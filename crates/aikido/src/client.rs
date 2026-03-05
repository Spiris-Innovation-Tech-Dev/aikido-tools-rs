use crate::config::{Credentials, Region};
use crate::error::*;
use base64::Engine;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, RwLock};

#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub jitter_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(1_500),
            jitter_ms: 100,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    #[allow(dead_code)]
    token_type: String,
}

#[derive(Debug, Clone, Deserialize)]
struct OAuthErrorResponse {
    error: String,
    error_description: Option<String>,
}

#[derive(Debug, Clone)]
struct CachedToken {
    access_token: String,
    expires_at: Instant,
}

// ========== Response Cache ==========

#[derive(Debug, Clone)]
struct CacheEntry {
    body: String,
    cached_at: Instant,
    ttl: Duration,
}

impl CacheEntry {
    fn is_valid(&self) -> bool {
        self.cached_at.elapsed() < self.ttl
    }
}

const TTL_ISSUES: Duration = Duration::from_secs(60);
const TTL_METADATA: Duration = Duration::from_secs(300);
const TTL_RULES: Duration = Duration::from_secs(600);
const TTL_COMPLIANCE: Duration = Duration::from_secs(300);
const RESPONSE_CACHE_MAX_ENTRIES: usize = 256;
const RESPONSE_CACHE_MAX_BODY_BYTES: usize = 256 * 1024;

pub(crate) fn encode_path_segment(segment: &str, field_name: &str) -> Result<String> {
    if segment.is_empty() {
        return Err(AikidoError::General(format!(
            "Invalid {field_name}: value must not be empty"
        )));
    }
    if segment == "." || segment == ".." || segment.contains('/') || segment.contains('\\') {
        return Err(AikidoError::General(format!(
            "Invalid {field_name}: path separators are not allowed"
        )));
    }
    if segment.chars().any(|ch| ch.is_control()) {
        return Err(AikidoError::General(format!(
            "Invalid {field_name}: control characters are not allowed"
        )));
    }
    Ok(urlencoding::encode(segment).into_owned())
}

fn ttl_for_path(path: &str) -> Duration {
    if path.starts_with("/issues") || path.starts_with("/open-issue-groups") {
        TTL_ISSUES
    } else if path.starts_with("/repositories/code/sast/rules")
        || path.starts_with("/repositories/code/iac/rules")
        || path.starts_with("/repositories/code/mobile/rules")
        || path.starts_with("/repositories/sast/custom-rules")
    {
        TTL_RULES
    } else if path.starts_with("/report") {
        TTL_COMPLIANCE
    } else {
        TTL_METADATA
    }
}

fn invalidation_prefixes(endpoint: &str) -> Vec<String> {
    let trimmed = endpoint.strip_prefix('/').unwrap_or(endpoint);
    let primary = match trimmed.split('/').next() {
        Some(first) => format!("/{first}"),
        None => endpoint.to_string(),
    };

    let mut prefixes = vec![primary.clone()];

    // Issue writes also invalidate the open-issue-groups list
    if primary == "/issues" {
        prefixes.push("/open-issue-groups".to_string());
    }

    prefixes
}

// ========== Client ==========

pub struct AikidoClient {
    http: reqwest::Client,
    region: Region,
    credentials: Credentials,
    token_cache: Arc<Mutex<Option<CachedToken>>>,
    retry_policy: RetryPolicy,
    response_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl AikidoClient {
    pub fn new(region: Region, credentials: Credentials) -> Self {
        Self::new_with_retry_policy(region, credentials, RetryPolicy::default())
    }

    pub fn new_with_retry_policy(
        region: Region,
        credentials: Credentials,
        retry_policy: RetryPolicy,
    ) -> Self {
        Self {
            http: reqwest::Client::new(),
            region,
            credentials,
            token_cache: Arc::new(Mutex::new(None)),
            retry_policy,
            response_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn region(&self) -> Region {
        self.region
    }

    // ---- Response cache helpers ----

    async fn cache_get(&self, path: &str) -> Option<String> {
        let mut cache = self.response_cache.write().await;
        if let Some(entry) = cache.get(path) {
            if entry.is_valid() {
                return Some(entry.body.clone());
            }
        }
        cache.remove(path);
        None
    }

    async fn cache_put(&self, path: &str, body: &str, ttl: Duration) {
        if body.len() > RESPONSE_CACHE_MAX_BODY_BYTES {
            return;
        }

        let mut cache = self.response_cache.write().await;
        cache.retain(|_, entry| entry.is_valid());

        while cache.len() >= RESPONSE_CACHE_MAX_ENTRIES {
            let Some(oldest_key) = cache
                .iter()
                .min_by_key(|(_, entry)| entry.cached_at)
                .map(|(key, _)| key.clone())
            else {
                break;
            };
            cache.remove(&oldest_key);
        }

        cache.insert(
            path.to_string(),
            CacheEntry {
                body: body.to_string(),
                cached_at: Instant::now(),
                ttl,
            },
        );
    }

    async fn invalidate_cache(&self, endpoint: &str) {
        let prefixes = invalidation_prefixes(endpoint);
        let mut cache = self.response_cache.write().await;
        cache.retain(|key, _| !prefixes.iter().any(|p| key.starts_with(p.as_str())));
    }

    pub async fn clear_cache(&self) {
        let mut cache = self.response_cache.write().await;
        cache.clear();
    }

    // ---- Token management ----

    async fn get_token(&self) -> Result<String> {
        match &self.credentials {
            Credentials::Token(token) => return Ok(token.clone()),
            Credentials::ClientCredentials {
                client_id,
                client_secret,
            } => {
                // Check cache
                {
                    let cache = self.token_cache.lock().await;
                    if let Some(cached) = cache.as_ref() {
                        if cached.expires_at > Instant::now() + Duration::from_secs(60) {
                            return Ok(cached.access_token.clone());
                        }
                    }
                }

                // Fetch new token
                let token_url = format!("{}/token", self.region.oauth_url());
                let basic_auth = base64::engine::general_purpose::STANDARD
                    .encode(format!("{client_id}:{client_secret}"));

                let response = self
                    .http
                    .post(&token_url)
                    .header("Authorization", format!("Basic {basic_auth}"))
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({ "grant_type": "client_credentials" }))
                    .send()
                    .await?;

                if !response.status().is_success() {
                    let status = response.status().as_u16();
                    let body = response.text().await.unwrap_or_default();
                    if let Ok(err) = serde_json::from_str::<OAuthErrorResponse>(&body) {
                        return Err(AikidoError::Auth(format!(
                            "{}: {}",
                            err.error,
                            err.error_description.unwrap_or_default()
                        )));
                    }
                    return Err(AikidoError::Api {
                        status,
                        error: "oauth_error".into(),
                        error_description: body,
                    });
                }

                let token_resp: TokenResponse = response.json().await?;

                let cached = CachedToken {
                    access_token: token_resp.access_token.clone(),
                    expires_at: Instant::now() + Duration::from_secs(token_resp.expires_in),
                };

                {
                    let mut cache = self.token_cache.lock().await;
                    *cache = Some(cached);
                }

                Ok(token_resp.access_token)
            }
        }
    }

    fn add_auth_headers(
        &self,
        request: reqwest::RequestBuilder,
        token: &str,
    ) -> reqwest::RequestBuilder {
        request
            .header("Authorization", format!("Bearer {token}"))
            .header("Accept", "application/json")
    }

    // ---- HTTP methods ----

    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        if let Some(cached) = self.cache_get(endpoint).await {
            return serde_json::from_str(&cached).map_err(|e| {
                AikidoError::General(format!("Failed to parse cached response: {e}"))
            });
        }

        let url = format!("{}{}", self.region.api_base_url(), endpoint);
        let token = self.get_token().await?;
        let response = self
            .send_with_retry(|| self.add_auth_headers(self.http.get(&url), &token))
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            self.cache_put(endpoint, &text, ttl_for_path(endpoint))
                .await;
            serde_json::from_str(&text).map_err(|e| {
                AikidoError::General(format!(
                    "Failed to parse response body as JSON: {e}. response_length={}",
                    text.len()
                ))
            })
        } else {
            parse_api_error(status.as_u16(), &text)
        }
    }

    pub async fn get_raw(&self, endpoint: &str) -> Result<String> {
        if let Some(cached) = self.cache_get(endpoint).await {
            return Ok(cached);
        }

        let url = format!("{}{}", self.region.api_base_url(), endpoint);
        let token = self.get_token().await?;
        let response = self
            .send_with_retry(|| self.add_auth_headers(self.http.get(&url), &token))
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            self.cache_put(endpoint, &text, ttl_for_path(endpoint))
                .await;
            Ok(text)
        } else {
            parse_api_error(status.as_u16(), &text)
        }
    }

    pub async fn get_bytes(&self, endpoint: &str) -> Result<Vec<u8>> {
        let url = format!("{}{}", self.region.api_base_url(), endpoint);
        let token = self.get_token().await?;
        let response = self
            .send_with_retry(|| self.add_auth_headers(self.http.get(&url), &token))
            .await?;

        if response.status().is_success() {
            Ok(response.bytes().await?.to_vec())
        } else {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            parse_api_error(status, &text)
        }
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.region.api_base_url(), endpoint);
        let token = self.get_token().await?;
        let request = self
            .add_auth_headers(self.http.post(&url), &token)
            .json(body);
        let response = request.send().await?;
        let result = self.handle_response(response).await;
        if result.is_ok() {
            self.invalidate_cache(endpoint).await;
        }
        result
    }

    pub async fn post_no_content<B>(&self, endpoint: &str, body: &B) -> Result<()>
    where
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.region.api_base_url(), endpoint);
        let token = self.get_token().await?;
        let request = self
            .add_auth_headers(self.http.post(&url), &token)
            .json(body);
        let response = request.send().await?;

        if response.status().is_success() {
            self.invalidate_cache(endpoint).await;
            Ok(())
        } else {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            parse_api_error(status, &text)
        }
    }

    pub async fn put<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.region.api_base_url(), endpoint);
        let token = self.get_token().await?;
        let request = self
            .add_auth_headers(self.http.put(&url), &token)
            .json(body);
        let response = request.send().await?;
        let result = self.handle_response(response).await;
        if result.is_ok() {
            self.invalidate_cache(endpoint).await;
        }
        result
    }

    pub async fn put_no_content<B>(&self, endpoint: &str, body: &B) -> Result<()>
    where
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.region.api_base_url(), endpoint);
        let token = self.get_token().await?;
        let request = self
            .add_auth_headers(self.http.put(&url), &token)
            .json(body);
        let response = request.send().await?;

        if response.status().is_success() {
            self.invalidate_cache(endpoint).await;
            Ok(())
        } else {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            parse_api_error(status, &text)
        }
    }

    pub async fn delete(&self, endpoint: &str) -> Result<()> {
        let url = format!("{}{}", self.region.api_base_url(), endpoint);
        let token = self.get_token().await?;
        let request = self.add_auth_headers(self.http.delete(&url), &token);
        let response = request.send().await?;

        if response.status().is_success() {
            self.invalidate_cache(endpoint).await;
            Ok(())
        } else {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            parse_api_error(status, &text)
        }
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            serde_json::from_str(&text).map_err(|e| {
                AikidoError::General(format!(
                    "Failed to parse response body as JSON: {e}. response_length={}",
                    text.len()
                ))
            })
        } else {
            parse_api_error(status.as_u16(), &text)
        }
    }

    async fn send_with_retry<F>(
        &self,
        build_request: F,
    ) -> std::result::Result<reqwest::Response, AikidoError>
    where
        F: Fn() -> reqwest::RequestBuilder,
    {
        let max_attempts = self.retry_policy.max_attempts.max(1);
        let mut attempt: u32 = 1;
        loop {
            let request = build_request();
            match request.send().await {
                Ok(response) => {
                    if is_retryable_status(response.status()) && attempt < max_attempts {
                        tokio::time::sleep(next_retry_delay(self.retry_policy, attempt)).await;
                        attempt = attempt.saturating_add(1);
                        continue;
                    }
                    return Ok(response);
                }
                Err(err) => {
                    if (err.is_timeout() || err.is_connect()) && attempt < max_attempts {
                        tokio::time::sleep(next_retry_delay(self.retry_policy, attempt)).await;
                        attempt = attempt.saturating_add(1);
                        continue;
                    }
                    return Err(AikidoError::Http(err));
                }
            }
        }
    }

    pub async fn workspace_info(&self) -> Result<crate::models::WorkspaceInfo> {
        self.get("/workspace").await
    }
}

fn parse_api_error<T>(status: u16, body: &str) -> Result<T> {
    #[derive(Deserialize)]
    struct ApiError {
        error: Option<String>,
        error_description: Option<String>,
        message: Option<String>,
    }

    if let Ok(err) = serde_json::from_str::<ApiError>(body) {
        Err(AikidoError::Api {
            status,
            error: err
                .error
                .or(err.message.clone())
                .unwrap_or_else(|| "unknown".into()),
            error_description: err
                .error_description
                .or(err.message)
                .unwrap_or_else(|| body.to_string()),
        })
    } else {
        Err(AikidoError::Api {
            status,
            error: "unknown".into(),
            error_description: body.to_string(),
        })
    }
}

fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(
        status,
        reqwest::StatusCode::REQUEST_TIMEOUT
            | reqwest::StatusCode::TOO_MANY_REQUESTS
            | reqwest::StatusCode::INTERNAL_SERVER_ERROR
            | reqwest::StatusCode::BAD_GATEWAY
            | reqwest::StatusCode::SERVICE_UNAVAILABLE
            | reqwest::StatusCode::GATEWAY_TIMEOUT
    )
}

fn next_retry_delay(policy: RetryPolicy, attempt: u32) -> Duration {
    let shift = attempt.saturating_sub(1).min(62);
    let factor = 1u64.checked_shl(shift).unwrap_or(u64::MAX);
    let exp_backoff = (policy.base_delay.as_millis() as u64)
        .saturating_mul(factor)
        .min(policy.max_delay.as_millis() as u64);
    let jitter = random_jitter(policy.jitter_ms);
    Duration::from_millis(exp_backoff.saturating_add(jitter))
}

fn random_jitter(max_jitter_ms: u64) -> u64 {
    if max_jitter_ms == 0 {
        return 0;
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (now.subsec_nanos() as u64) % (max_jitter_ms + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ttl_for_issues() {
        assert_eq!(ttl_for_path("/issues/export"), TTL_ISSUES);
        assert_eq!(ttl_for_path("/issues/123"), TTL_ISSUES);
        assert_eq!(ttl_for_path("/issues/groups/456"), TTL_ISSUES);
        assert_eq!(ttl_for_path("/issues/counts"), TTL_ISSUES);
        assert_eq!(ttl_for_path("/open-issue-groups"), TTL_ISSUES);
    }

    #[test]
    fn ttl_for_metadata() {
        assert_eq!(ttl_for_path("/repositories/code"), TTL_METADATA);
        assert_eq!(ttl_for_path("/containers"), TTL_METADATA);
        assert_eq!(ttl_for_path("/clouds"), TTL_METADATA);
        assert_eq!(ttl_for_path("/domains"), TTL_METADATA);
        assert_eq!(ttl_for_path("/teams"), TTL_METADATA);
        assert_eq!(ttl_for_path("/users"), TTL_METADATA);
        assert_eq!(ttl_for_path("/workspace"), TTL_METADATA);
        assert_eq!(ttl_for_path("/firewall/apps"), TTL_METADATA);
    }

    #[test]
    fn ttl_for_rules() {
        assert_eq!(ttl_for_path("/repositories/code/sast/rules"), TTL_RULES);
        assert_eq!(ttl_for_path("/repositories/code/iac/rules"), TTL_RULES);
        assert_eq!(ttl_for_path("/repositories/code/mobile/rules"), TTL_RULES);
        assert_eq!(ttl_for_path("/repositories/sast/custom-rules"), TTL_RULES);
        assert_eq!(
            ttl_for_path("/repositories/sast/custom-rules/42"),
            TTL_RULES
        );
    }

    #[test]
    fn ttl_for_compliance() {
        assert_eq!(ttl_for_path("/report/iso/overview"), TTL_COMPLIANCE);
        assert_eq!(ttl_for_path("/report/activityLog"), TTL_COMPLIANCE);
        assert_eq!(ttl_for_path("/report/ciScans"), TTL_COMPLIANCE);
    }

    #[test]
    fn invalidation_prefix_extraction() {
        assert_eq!(
            invalidation_prefixes("/issues/123/ignore"),
            vec!["/issues", "/open-issue-groups"]
        );
        assert_eq!(
            invalidation_prefixes("/issues/groups/456/snooze"),
            vec!["/issues", "/open-issue-groups"]
        );
        assert_eq!(invalidation_prefixes("/teams/5/addUser"), vec!["/teams"]);
        assert_eq!(invalidation_prefixes("/containers/42"), vec!["/containers"]);
        assert_eq!(
            invalidation_prefixes("/repositories/code/activate"),
            vec!["/repositories"]
        );
    }

    #[test]
    fn cache_entry_validity() {
        let entry = CacheEntry {
            body: "test".to_string(),
            cached_at: Instant::now(),
            ttl: Duration::from_secs(60),
        };
        assert!(entry.is_valid());

        let expired = CacheEntry {
            body: "test".to_string(),
            cached_at: Instant::now() - Duration::from_secs(120),
            ttl: Duration::from_secs(60),
        };
        assert!(!expired.is_valid());
    }

    #[test]
    fn encode_path_segment_rejects_unsafe_values() {
        assert!(encode_path_segment("", "id").is_err());
        assert!(encode_path_segment("..", "id").is_err());
        assert!(encode_path_segment("foo/bar", "id").is_err());
        assert!(encode_path_segment("foo\\bar", "id").is_err());
    }

    #[test]
    fn encode_path_segment_escapes_reserved_chars() {
        assert_eq!(
            encode_path_segment("project 1", "project_id").unwrap(),
            "project%201"
        );
        assert_eq!(encode_path_segment("a?b", "project_id").unwrap(), "a%3Fb");
    }
}

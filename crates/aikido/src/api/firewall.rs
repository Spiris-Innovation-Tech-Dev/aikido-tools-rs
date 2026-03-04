use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn list_firewall_apps(&self) -> Result<Vec<FirewallApp>> {
        self.get("/firewall/apps").await
    }

    pub async fn get_firewall_app(&self, app_id: i64) -> Result<FirewallApp> {
        self.get(&format!("/firewall/apps/{app_id}")).await
    }

    pub async fn create_firewall_app(&self, body: &CreateAppRequest) -> Result<FirewallApp> {
        self.post("/firewall/apps", body).await
    }

    pub async fn update_firewall_app(
        &self,
        app_id: i64,
        body: &UpdateAppRequest,
    ) -> Result<Value> {
        self.put(&format!("/firewall/apps/{app_id}"), body).await
    }

    pub async fn delete_firewall_app(&self, app_id: i64) -> Result<()> {
        self.delete(&format!("/firewall/apps/{app_id}")).await
    }

    pub async fn get_bot_lists(&self, app_id: i64) -> Result<BotLists> {
        self.get(&format!("/firewall/apps/{app_id}/bot-lists"))
            .await
    }

    pub async fn update_bot_lists(&self, app_id: i64, body: &Value) -> Result<Value> {
        self.put(&format!("/firewall/apps/{app_id}/bot-lists"), body)
            .await
    }

    pub async fn get_countries(&self, app_id: i64) -> Result<Countries> {
        self.get(&format!("/firewall/apps/{app_id}/countries"))
            .await
    }

    pub async fn update_countries(&self, app_id: i64, body: &Value) -> Result<Value> {
        self.put(&format!("/firewall/apps/{app_id}/countries"), body)
            .await
    }

    pub async fn get_firewall_event(
        &self,
        app_id: i64,
        event_id: i64,
    ) -> Result<FirewallEvent> {
        self.get(&format!("/firewall/apps/{app_id}/events/{event_id}"))
            .await
    }

    pub async fn update_ip_blocklist(&self, app_id: i64, body: &Value) -> Result<Value> {
        self.put(&format!("/firewall/apps/{app_id}/ip-blocklist"), body)
            .await
    }

    pub async fn get_ip_lists(&self, app_id: i64) -> Result<IpLists> {
        self.get(&format!("/firewall/apps/{app_id}/ip-lists"))
            .await
    }

    pub async fn update_ip_lists(&self, app_id: i64, body: &Value) -> Result<Value> {
        self.put(&format!("/firewall/apps/{app_id}/ip-lists"), body)
            .await
    }

    pub async fn rotate_app_token(&self, app_id: i64) -> Result<Value> {
        self.post(
            &format!("/firewall/apps/{app_id}/token"),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn update_blocking(
        &self,
        service_id: i64,
        body: &UpdateBlockingRequest,
    ) -> Result<Value> {
        self.put(&format!("/firewall/apps/{service_id}/blocking"), body)
            .await
    }

    pub async fn update_firewall_user(
        &self,
        app_id: i64,
        user_id: i64,
        body: &UpdateUserRequest,
    ) -> Result<Value> {
        self.put(&format!("/firewall/{app_id}/users/{user_id}"), body)
            .await
    }
}

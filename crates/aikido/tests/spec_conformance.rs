use aikido::models::{AdjustSeverityRequest, CreateDomainRequest, UpdateBlockingRequest};
use serde_json::Value;
use std::collections::HashMap;

fn rest_spec() -> Value {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../aikido-rest.bundled.json"
    )))
    .expect("valid bundled REST OpenAPI spec")
}

fn operation_map(spec: &Value) -> HashMap<String, (String, String)> {
    let mut out = HashMap::new();
    let paths = spec
        .get("paths")
        .and_then(Value::as_object)
        .expect("spec.paths object must exist");

    for (path, methods) in paths {
        let Some(methods_obj) = methods.as_object() else {
            continue;
        };
        for (method, op) in methods_obj {
            let Some(op_id) = op.get("operationId").and_then(Value::as_str) else {
                continue;
            };
            out.insert(op_id.to_string(), (path.to_string(), method.to_string()));
        }
    }

    out
}

fn required_fields_for_operation(spec: &Value, operation_id: &str) -> Vec<String> {
    let paths = spec
        .get("paths")
        .and_then(Value::as_object)
        .expect("spec.paths object must exist");

    for methods in paths.values() {
        let Some(methods_obj) = methods.as_object() else {
            continue;
        };
        for op in methods_obj.values() {
            let Some(op_id) = op.get("operationId").and_then(Value::as_str) else {
                continue;
            };
            if op_id != operation_id {
                continue;
            }

            return op
                .get("requestBody")
                .and_then(|b| b.get("content"))
                .and_then(|c| c.get("application/json"))
                .and_then(|j| j.get("schema"))
                .and_then(|s| s.get("required"))
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
        }
    }

    Vec::new()
}

#[test]
fn operation_id_path_parity_for_core_sdk_calls() {
    let spec = rest_spec();
    let ops = operation_map(&spec);

    let expected = [
        ("listOpenIssueGroups", "/open-issue-groups", "get"),
        ("getIssueCounts", "/issues/counts", "get"),
        ("exportIssues", "/issues/export", "get"),
        ("ignoreIssue", "/issues/{issue_id}/ignore", "put"),
        ("UnignoreIssue", "/issues/{issue_id}/unignore", "put"),
        ("snoozeIssue", "/issues/{issue_id}/snooze", "put"),
        ("UnsnoozeIssue", "/issues/{issue_id}/unsnooze", "put"),
        (
            "getIssueGroupDetails",
            "/issues/groups/{issue_group_id}",
            "get",
        ),
        (
            "updateBlocking",
            "/firewall/apps/{service_id}/blocking",
            "put",
        ),
        ("createDomain", "/domains", "post"),
        (
            "updateDomainOpenAPISpec",
            "/domains/{domain_id}/update/openapi-spec",
            "put",
        ),
    ];

    for (operation_id, path, method) in expected {
        let (actual_path, actual_method) = ops
            .get(operation_id)
            .unwrap_or_else(|| panic!("missing operationId in spec: {operation_id}"));
        assert_eq!(actual_path, path, "path drift for {operation_id}");
        assert_eq!(actual_method, method, "method drift for {operation_id}");
    }
}

#[test]
fn adjust_severity_request_matches_spec_required_fields() {
    let spec = rest_spec();
    let required = required_fields_for_operation(&spec, "adjustSeverity");
    let body = AdjustSeverityRequest {
        adjusted_severity: "high".to_string(),
        reason: "risk acceptance changed".to_string(),
    };
    let serialized = serde_json::to_value(body).expect("serializes");

    for field in required {
        assert!(
            serialized.get(&field).is_some(),
            "missing required field in payload: {field}"
        );
    }
}

#[test]
fn create_domain_request_matches_spec_required_fields() {
    let spec = rest_spec();
    let required = required_fields_for_operation(&spec, "createDomain");
    let body = CreateDomainRequest {
        domain: "https://www.example.com".to_string(),
        kind: "front_end".to_string(),
        zen_service_id: None,
        openapi_spec_base64: None,
        openapi_spec_url: None,
    };
    let serialized = serde_json::to_value(body).expect("serializes");

    for field in required {
        assert!(
            serialized.get(&field).is_some(),
            "missing required field in payload: {field}"
        );
    }
}

#[test]
fn update_blocking_request_matches_spec_required_fields() {
    let spec = rest_spec();
    let required = required_fields_for_operation(&spec, "updateBlocking");
    let body = UpdateBlockingRequest {
        block: true,
        disable_minimum_wait_check: Some(false),
    };
    let serialized = serde_json::to_value(body).expect("serializes");

    for field in required {
        assert!(
            serialized.get(&field).is_some(),
            "missing required field in payload: {field}"
        );
    }
}

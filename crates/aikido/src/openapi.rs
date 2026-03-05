use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct OpenApiOperation {
    pub operation_id: String,
    pub method: String,
    pub path: String,
}

fn spec() -> Value {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../aikido-rest.bundled.json"
    )))
    .expect("valid bundled OpenAPI spec")
}

pub fn list_operations() -> Vec<OpenApiOperation> {
    let mut out = Vec::new();
    let doc = spec();
    let Some(paths) = doc.get("paths").and_then(Value::as_object) else {
        return out;
    };

    for (path, methods) in paths {
        let Some(methods_obj) = methods.as_object() else {
            continue;
        };

        for (method, op) in methods_obj {
            let Some(op_id) = op.get("operationId").and_then(Value::as_str) else {
                continue;
            };

            out.push(OpenApiOperation {
                operation_id: op_id.to_string(),
                method: method.to_ascii_lowercase(),
                path: path.to_string(),
            });
        }
    }

    out
}

pub fn find_operation(operation_id: &str) -> Option<OpenApiOperation> {
    list_operations()
        .into_iter()
        .find(|op| op.operation_id == operation_id)
}

pub fn render_path(path_template: &str, path_params: &[(String, String)]) -> String {
    let mut out = path_template.to_string();
    for (k, v) in path_params {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out
}

pub fn append_query(path: &str, query_params: &[(String, String)]) -> String {
    if query_params.is_empty() {
        return path.to_string();
    }

    let mut out = path.to_string();
    let sep = if out.contains('?') { '&' } else { '?' };
    out.push(sep);
    out.push_str(
        &query_params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&"),
    );
    out
}

#[derive(Debug, Serialize)]
pub struct OperationExecuteInfo {
    pub operation_id: String,
    pub method: String,
    pub endpoint: String,
}

use rmcp::model::{ErrorCode, ErrorData};

#[allow(dead_code)]
pub fn aikido_error_to_mcp(err: aikido::error::AikidoError) -> ErrorData {
    ErrorData::new(ErrorCode(-32603), err.to_string(), None)
}

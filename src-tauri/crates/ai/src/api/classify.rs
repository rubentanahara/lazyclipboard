use lazyclipboard_core::model::AiError;
use serde_json::Value;

const RETRY_AFTER_HEADER: &str = "retry-after";
const INVALID_KEY_CODE: &str = "invalid_api_key";
const GEMINI_INVALID_KEY_REASON: &str = "API_KEY_INVALID";
const ANTHROPIC_SPEND_LIMIT_CODE: &str = "enforced_spend_limit_reached";
const OPENAI_BILLING_CODES: [&str; 4] = [
    "credit_balance_exhausted",
    "organization_spend_limit_exceeded",
    "project_spend_limit_exceeded",
    "organization_usage_limit_exceeded",
];
const OVERLOADED_MARKERS: [&str; 2] = ["overloaded_error", "server_is_overloaded"];
const STATUS_UNAUTHORIZED: u16 = 401;
const STATUS_PAYMENT_REQUIRED: u16 = 402;
const STATUS_TOO_MANY_REQUESTS: u16 = 429;
const STATUS_SERVICE_UNAVAILABLE: u16 = 503;
const STATUS_ANTHROPIC_OVERLOADED: u16 = 529;

pub fn classify_error(status: u16, headers: &[(&str, &str)], body: &str) -> AiError {
    let body: Value = serde_json::from_str(body).unwrap_or(Value::Null);

    if is_invalid_key(&body) || status == STATUS_UNAUTHORIZED {
        return AiError::InvalidKey;
    }
    if is_quota_exhausted(&body) || status == STATUS_PAYMENT_REQUIRED {
        return AiError::QuotaExhausted;
    }
    if status == STATUS_TOO_MANY_REQUESTS {
        return AiError::RateLimited {
            retry_after_secs: retry_after_secs(headers),
        };
    }
    if is_overloaded(status, &body) {
        return AiError::Overloaded;
    }
    AiError::Other {
        message: error_message(&body).unwrap_or_else(|| format!("HTTP {status}")),
    }
}

fn is_invalid_key(body: &Value) -> bool {
    let has_invalid_key_reason = body["error"]["details"].as_array().is_some_and(|details| {
        details
            .iter()
            .any(|detail| detail["reason"] == GEMINI_INVALID_KEY_REASON)
    });
    has_invalid_key_reason || body["error"]["code"] == INVALID_KEY_CODE
}

fn is_quota_exhausted(body: &Value) -> bool {
    let is_openai_billing = OPENAI_BILLING_CODES
        .iter()
        .any(|code| body["error"]["code"] == *code);
    is_openai_billing || body["error"]["details"]["error_code"] == ANTHROPIC_SPEND_LIMIT_CODE
}

fn is_overloaded(status: u16, body: &Value) -> bool {
    let has_overloaded_marker = OVERLOADED_MARKERS
        .iter()
        .any(|marker| body["error"]["type"] == *marker || body["error"]["code"] == *marker);
    has_overloaded_marker
        || status == STATUS_SERVICE_UNAVAILABLE
        || status == STATUS_ANTHROPIC_OVERLOADED
}

fn retry_after_secs(headers: &[(&str, &str)]) -> Option<u32> {
    headers
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(RETRY_AFTER_HEADER))
        .and_then(|(_, value)| value.trim().parse().ok())
}

fn error_message(body: &Value) -> Option<String> {
    body["error"]["message"].as_str().map(str::to_owned)
}

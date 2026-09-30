use std::future::{ready, Future};

use lazyclipboard_core::model::AiError;

use super::{classify_error, AiProvider};

const ANTHROPIC_401: &str = include_str!("fixtures/anthropic_401_authentication.json");
const ANTHROPIC_429_RATE_LIMIT: &str = include_str!("fixtures/anthropic_429_rate_limit.json");
const ANTHROPIC_429_SPEND_LIMIT: &str = include_str!("fixtures/anthropic_429_spend_limit.json");
const ANTHROPIC_529: &str = include_str!("fixtures/anthropic_529_overloaded.json");
const OPENAI_401: &str = include_str!("fixtures/openai_401_invalid_key.json");
const OPENAI_429_SLOW_DOWN: &str = include_str!("fixtures/openai_429_slow_down.json");
const OPENAI_429_CREDIT: &str = include_str!("fixtures/openai_429_credit_balance_exhausted.json");
const OPENAI_503: &str = include_str!("fixtures/openai_503_overloaded.json");
const GEMINI_400_INVALID_KEY: &str = include_str!("fixtures/gemini_400_invalid_key.json");

const NO_HEADERS: &[(&str, &str)] = &[];

#[test]
fn recorded_429_body_returns_the_rate_limit_variant() {
    let error = classify_error(429, &[("retry-after", "30")], ANTHROPIC_429_RATE_LIMIT);

    assert_eq!(
        error,
        AiError::RateLimited {
            retry_after_secs: Some(30)
        }
    );
}

#[test]
fn rate_limit_without_retry_after_has_no_wait() {
    let error = classify_error(429, NO_HEADERS, OPENAI_429_SLOW_DOWN);

    assert_eq!(
        error,
        AiError::RateLimited {
            retry_after_secs: None
        }
    );
}

#[test]
fn retry_after_header_name_is_case_insensitive() {
    let error = classify_error(429, &[("Retry-After", "7")], OPENAI_429_SLOW_DOWN);

    assert_eq!(
        error,
        AiError::RateLimited {
            retry_after_secs: Some(7)
        }
    );
}

#[test]
fn retry_after_http_date_is_not_a_number_of_seconds() {
    let headers = [("retry-after", "Wed, 21 Oct 2026 07:28:00 GMT")];

    let error = classify_error(429, &headers, ANTHROPIC_429_RATE_LIMIT);

    assert_eq!(
        error,
        AiError::RateLimited {
            retry_after_secs: None
        }
    );
}

#[test]
fn rate_limit_with_unparseable_body_is_still_rate_limited() {
    let error = classify_error(429, NO_HEADERS, "<html>Too Many Requests</html>");

    assert_eq!(
        error,
        AiError::RateLimited {
            retry_after_secs: None
        }
    );
}

#[test]
fn anthropic_spend_cap_429_is_quota_exhausted() {
    let error = classify_error(429, NO_HEADERS, ANTHROPIC_429_SPEND_LIMIT);

    assert_eq!(error, AiError::QuotaExhausted);
}

#[test]
fn openai_billing_429_is_quota_exhausted() {
    let error = classify_error(429, NO_HEADERS, OPENAI_429_CREDIT);

    assert_eq!(error, AiError::QuotaExhausted);
}

#[test]
fn payment_required_is_quota_exhausted() {
    let error = classify_error(402, NO_HEADERS, "{}");

    assert_eq!(error, AiError::QuotaExhausted);
}

#[test]
fn anthropic_and_openai_401_are_invalid_key() {
    assert_eq!(
        classify_error(401, NO_HEADERS, ANTHROPIC_401),
        AiError::InvalidKey
    );
    assert_eq!(
        classify_error(401, NO_HEADERS, OPENAI_401),
        AiError::InvalidKey
    );
}

#[test]
fn gemini_400_with_api_key_invalid_reason_is_invalid_key() {
    let error = classify_error(400, NO_HEADERS, GEMINI_400_INVALID_KEY);

    assert_eq!(error, AiError::InvalidKey);
}

#[test]
fn anthropic_529_and_openai_503_are_overloaded() {
    assert_eq!(
        classify_error(529, NO_HEADERS, ANTHROPIC_529),
        AiError::Overloaded
    );
    assert_eq!(
        classify_error(503, NO_HEADERS, OPENAI_503),
        AiError::Overloaded
    );
}

#[test]
fn overloaded_body_is_overloaded_whatever_the_status() {
    let error = classify_error(500, NO_HEADERS, ANTHROPIC_529);

    assert_eq!(error, AiError::Overloaded);
}

#[test]
fn unknown_failure_carries_the_provider_message() {
    let body = r#"{"error":{"message":"model not found"}}"#;

    let error = classify_error(404, NO_HEADERS, body);

    assert_eq!(
        error,
        AiError::Other {
            message: "model not found".to_string()
        }
    );
}

#[test]
fn unknown_failure_without_a_message_names_the_status() {
    let error = classify_error(500, NO_HEADERS, "");

    assert_eq!(
        error,
        AiError::Other {
            message: "HTTP 500".to_string()
        }
    );
}

struct FakeProvider;

impl AiProvider for FakeProvider {
    fn default_model(&self) -> &str {
        "fake-model"
    }

    fn complete(
        &self,
        prompt: &str,
        text: &str,
    ) -> impl Future<Output = Result<String, AiError>> + Send {
        ready(Ok(format!("{prompt}{text}")))
    }

    fn test_connection(&self) -> impl Future<Output = Result<(), AiError>> + Send {
        ready(Err(AiError::NoKey))
    }
}

#[test]
fn provider_trait_exposes_the_default_model() {
    assert_eq!(FakeProvider.default_model(), "fake-model");
}

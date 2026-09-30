use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize, Deserialize, Type)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AiError {
    #[error("no API key is stored for the selected provider")]
    NoKey,
    #[error("the provider rejected the API key")]
    InvalidKey,
    #[error("the provider rate limited the request")]
    RateLimited { retry_after_secs: Option<u32> },
    #[error("the provider account has no quota left")]
    QuotaExhausted,
    #[error("the provider is overloaded")]
    Overloaded,
    #[error("the provider refused the content")]
    Blocked,
    #[error("the provider could not be reached")]
    Network,
    #[error("the provider returned an unexpected error: {message}")]
    Other { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize, Deserialize, Type)]
#[serde(tag = "kind", content = "detail", rename_all = "snake_case")]
pub enum CommandError {
    #[error("not found")]
    NotFound,
    #[error("invalid value for {field}")]
    Validation { field: String },
    #[error("conflict")]
    Conflict,
    #[error("a required permission is missing")]
    PermissionMissing,
    #[error("the target app runs elevated")]
    TargetElevated,
    #[error("nothing is selected")]
    NothingSelected,
    #[error("not supported on this system")]
    Unsupported,
    #[error("too large")]
    TooLarge,
    #[error("the keychain is unavailable")]
    KeychainUnavailable,
    #[error("AI is locked")]
    AiLocked,
    #[error(transparent)]
    Ai(#[from] AiError),
    #[error("internal error")]
    Internal,
}

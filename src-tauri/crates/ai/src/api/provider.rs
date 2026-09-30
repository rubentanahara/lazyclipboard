use std::future::Future;

use lazyclipboard_core::model::AiError;

pub trait AiProvider {
    fn default_model(&self) -> &str;

    fn complete(
        &self,
        prompt: &str,
        text: &str,
    ) -> impl Future<Output = Result<String, AiError>> + Send;

    fn test_connection(&self) -> impl Future<Output = Result<(), AiError>> + Send;
}

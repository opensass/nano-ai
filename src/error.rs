use serde_wasm_bindgen::Error as SerdeError;
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum NanoAIError {
    #[error("The requested model is unavailable.")]
    ModelUnavailable,

    #[error(
        "Browser or platform is not supported. Consider using Chrome Canary on a Windows machine."
    )]
    BrowserNotSupported,

    #[error("Failed to create session {0}")]
    SessionCreationFailed(String),

    #[error("Prompt execution failed: {0}")]
    PromptFailed(String),

    #[error("A serialization error occurred: {0}")]
    SerializationError(SerdeError),

    #[error("An unexpected JavaScript error occurred: {0:?}")]
    UnexpectedJsError(JsValue),
}

impl From<SerdeError> for NanoAIError {
    fn from(err: SerdeError) -> Self {
        NanoAIError::SerializationError(err)
    }
}

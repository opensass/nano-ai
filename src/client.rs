#![allow(unused)]
#![allow(deprecated)]

use crate::{adapter::*, error::NanoAIError, types::*};
use js_sys::{Array, Object};
use serde_wasm_bindgen::from_value;
use wasm_bindgen_futures::JsFuture;

pub struct NanoAI {
    session: Option<Session>,
}

impl Default for NanoAI {
    fn default() -> Self {
        Self::new()
    }
}

impl NanoAI {
    pub fn new() -> Self {
        NanoAI { session: None }
    }

    pub async fn get_capabilities(&self) -> Result<Capabilities, NanoAIError> {
        if !AI.is_undefined() {
            let promise = AI.language_model().capabilities();
            let result = JsFuture::from(promise).await;
            match result {
                Ok(value) => Ok(from_value(value)?),
                Err(err) => Err(NanoAIError::UnexpectedJsError(err)),
            }
        } else {
            Err(NanoAIError::BrowserNotSupported)
        }
    }

    pub async fn create_session(&mut self, _options: Option<Object>) -> Result<(), NanoAIError> {
        if !AI.is_undefined() {
            let session_promise = AI.language_model().create();
            let result = JsFuture::from(session_promise).await;

            match result {
                Ok(value) => {
                    self.session = Some(value.into());
                    Ok(())
                }
                Err(_err) => Err(NanoAIError::SessionCreationFailed("".to_string())),
            }
        } else {
            Err(NanoAIError::BrowserNotSupported)
        }
    }

    pub async fn send_prompt(&self, input: &str) -> Result<String, NanoAIError> {
        if !AI.is_undefined() {
            if let Some(session) = &self.session {
                let promise = session.prompt(input);
                let result = JsFuture::from(promise).await;

                match result {
                    Ok(value) => Ok(from_value(value)?),
                    Err(err) => Err(NanoAIError::PromptFailed(format!("{:?}", err))),
                }
            } else {
                Err(NanoAIError::SessionCreationFailed(
                    "Session Creation Failed!".to_string(),
                ))
            }
        } else {
            Err(NanoAIError::BrowserNotSupported)
        }
    }

    pub async fn stream_prompt(&self, input: &str) -> Result<Vec<StreamingChunk>, NanoAIError> {
        if !AI.is_undefined() {
            if let Some(session) = &self.session {
                let promise = session.prompt_streaming(input);
                let stream = JsFuture::from(promise).await;

                match stream {
                    Ok(value) => {
                        let array: Array = value.into();
                        let chunks = array
                            .iter()
                            .map(from_value::<StreamingChunk>)
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(chunks)
                    }
                    Err(err) => Err(NanoAIError::PromptFailed(format!("{:?}", err))),
                }
            } else {
                Err(NanoAIError::SessionCreationFailed(
                    "Session Creation Failed!".to_string(),
                ))
            }
        } else {
            Err(NanoAIError::BrowserNotSupported)
        }
    }

    pub async fn destroy_session(&mut self) -> Result<(), NanoAIError> {
        if !AI.is_undefined() {
            if let Some(session) = &self.session {
                let promise = session.destroy();
                JsFuture::from(promise)
                    .await
                    .map_err(NanoAIError::UnexpectedJsError)?;
                self.session = None;
                Ok(())
            } else {
                Err(NanoAIError::SessionCreationFailed(
                    "Session Creation Failed!".to_string(),
                ))
            }
        } else {
            Err(NanoAIError::BrowserNotSupported)
        }
    }
}

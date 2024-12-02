#![allow(unused)]
#![allow(deprecated)]

use js_sys::{Object, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = ai)]
    pub static AI: Ai;

    #[wasm_bindgen(js_namespace = ai, js_name = AILanguageModelFactory)]
    pub static AILANGUAGE_MODEL_FACTORY: ModelFactory;

    #[wasm_bindgen(js_namespace = ai, js_name = AILanguageModel)]
    pub static AILANGUAGE_MODEL: Session;

    #[wasm_bindgen(extends = Object)]
    pub type Ai;

    #[wasm_bindgen(extends = Object)]
    pub type ModelFactory;

    #[wasm_bindgen(extends = Object)]
    pub type Session;

    #[wasm_bindgen(method, getter, js_name = languageModel)]
    pub fn language_model(this: &Ai) -> ModelFactory;

    #[wasm_bindgen(method, js_name = capabilities)]
    pub fn capabilities(this: &ModelFactory) -> Promise;

    #[wasm_bindgen(method, js_name = create)]
    pub fn create(this: &ModelFactory) -> Promise;

    #[wasm_bindgen(method, js_name = prompt)]
    pub fn prompt(this: &Session, input: &str) -> Promise;

    #[wasm_bindgen(method, js_name = promptStreaming)]
    pub fn prompt_streaming(this: &Session, input: &str) -> Promise;

    #[wasm_bindgen(method, js_name = destroy)]
    pub fn destroy(this: &Session) -> Promise;

    #[wasm_bindgen(method, js_name = clone)]
    pub fn clone_session(this: &Session) -> Promise;
}

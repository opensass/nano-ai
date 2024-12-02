use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Capabilities {
    pub available: String,
    #[serde(rename = "defaultTemperature")]
    pub default_temperature: u32,
    #[serde(rename = "defaultTopK")]
    pub default_top_k: u32,
    #[serde(rename = "maxTopK")]
    pub max_top_k: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamingChunk {
    pub text: String,
}

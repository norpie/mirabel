use crate::prelude::*;
use chrono::{DateTime, Duration, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;
use std::collections::HashMap;


#[derive(Debug, Display, Serialize)]
#[display("{}:{}", model, tag)]
pub struct OllamaModel {
    model: String,
    tag: String,
}

impl OllamaModel {
    pub fn new(model: String) -> Self {
        if model.contains(':') {
            let mut parts = model.split(':');
            let model = parts.next().unwrap().to_string();
            let tag = parts.next().unwrap().to_string();
            return Self { model, tag };
        }
        Self {
            model,
            tag: "latest".into(),
        }
    }

    pub fn with_tag(model: String, tag: String) -> Self {
        Self { model, tag }
    }
}

impl From<String> for OllamaModel {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for OllamaModel {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}

impl From<(&str, &str)> for OllamaModel {
    fn from(value: (&str, &str)) -> Self {
        Self::with_tag(value.0.into(), value.1.into())
    }
}

#[derive(Debug, Display, Serialize)]
enum KeepAliveInner {
    #[display("{0}s", "_0.display()")]
    Seconds(usize),
    #[display("{0}m", "_0.display()")]
    Minutes(usize),
    #[display("{0}h", "_0.display()")]
    Hours(usize),
}

#[derive(Debug, Serialize)]
pub struct KeepAlive(KeepAliveInner);

impl KeepAlive {
    pub fn seconds(seconds: usize) -> Result<Self> {
        if seconds >= 60 {
            return Err("Seconds must be less than 60".into());
        }
        Ok(Self(KeepAliveInner::Seconds(seconds)))
    }

    pub fn minutes(minutes: usize) -> Result<Self> {
        if minutes >= 60 {
            return Err("Minutes must be less than 60".into());
        }
        Ok(Self(KeepAliveInner::Minutes(minutes)))
    }

    pub fn hours(hours: usize) -> Result<Self> {
        if hours >= 24 {
            return Err("Hours must be less than 24".into());
        }
        Ok(Self(KeepAliveInner::Hours(hours)))
    }
}

#[derive(Debug, Serialize)]
pub struct GenerateRequest {
    model: OllamaModel,
    prompt: String,
    suffix: Option<String>,
    images: Option<Vec<String>>,
    format: Option<String>,
    options: Option<HashMap<String, Value>>,
    system: Option<String>,
    template: Option<String>,
    raw: Option<bool>,
    keep_alive: Option<KeepAlive>,
}

impl GenerateRequest {
    pub fn new(model: OllamaModel, prompt: String) -> Self {
        Self {
            model,
            prompt,
            suffix: None,
            images: None,
            format: None,
            options: None,
            system: None,
            template: None,
            raw: None,
            keep_alive: None,
        }
    }

    pub fn suffix(mut self, suffix: String) -> Self {
        self.suffix = Some(suffix);
        self
    }

    pub fn images(mut self, images: Vec<String>) -> Self {
        self.images = Some(images);
        self
    }

    pub fn format(mut self, format: String) -> Self {
        self.format = Some(format);
        self
    }

    pub fn options(mut self, options: HashMap<String, Value>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn system(mut self, system: String) -> Self {
        self.system = Some(system);
        self
    }

    pub fn template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }

    pub fn raw(mut self, raw: bool) -> Self {
        self.raw = Some(raw);
        self
    }

    pub fn keep_alive(mut self, keep_alive: KeepAlive) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }
}

impl From<GenerateRequest> for GenerateRequestInternal {
    fn from(value: GenerateRequest) -> Self {
        Self {
            model: value.model.to_string(),
            prompt: Some(value.prompt),
            suffix: value.suffix,
            images: value.images,
            format: value.format,
            options: value.options,
            system: value.system,
            template: value.template,
            stream: Some(false),
            raw: value.raw,
            keep_alive: value.keep_alive.map(|x| x.0.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GenerateRequestInternal {
    model: String,
    prompt: Option<String>,
    suffix: Option<String>,
    images: Option<Vec<String>>,
    format: Option<String>,
    options: Option<HashMap<String, Value>>,
    system: Option<String>,
    template: Option<String>,
    stream: Option<bool>,
    raw: Option<bool>,
    keep_alive: Option<String>,
}

impl GenerateRequestInternal {
    pub fn with_stream(mut self) -> Self {
        self.stream = Some(true);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamGenerateResponse {
    pub model: String,
    pub created_at: DateTime<Utc>,
    pub response: String,
    pub done: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub done_reason: Option<DoneReason>,
    pub context: Option<Vec<i64>>,
    #[serde_as(as = "Option<serde_with::DurationNanoSeconds<i64>>")]
    pub total_duration: Option<Duration>,
    #[serde_as(as = "Option<serde_with::DurationNanoSeconds<i64>>")]
    pub load_duration: Option<Duration>,
    pub prompt_eval_count: Option<u32>,
    #[serde_as(as = "Option<serde_with::DurationNanoSeconds<i64>>")]
    pub prompt_eval_duration: Option<Duration>,
    pub eval_count: Option<u32>,
    #[serde_as(as = "Option<serde_with::DurationNanoSeconds<i64>>")]
    pub eval_duration: Option<Duration>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum DoneReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "load")]
    Load,
}

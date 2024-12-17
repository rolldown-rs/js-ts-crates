// https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview#heading=h.yr4qxyxotyw

use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::ops::Deref;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum TraceEventType {
    #[serde(rename = "B")]
    DurationBegin,
    #[serde(rename = "E")]
    DurationEnd,
    #[serde(rename = "X")]
    Complete,
    #[default]
    #[serde(rename = "i", alias = "I")]
    Instant,
    #[serde(rename = "C")]
    Counter,
    #[serde(rename = "b")]
    AsyncStart,
    #[serde(rename = "n")]
    AsyncInstant,
    #[serde(rename = "e")]
    AsyncEnd,
    #[serde(rename = "s")]
    FlowStart,
    #[serde(rename = "t")]
    FlowStep,
    #[serde(rename = "f")]
    FlowEnd,
    #[serde(rename = "P")]
    Sample,
    #[serde(rename = "N")]
    ObjectCreated,
    #[serde(rename = "O")]
    ObjectSnapshot,
    #[serde(rename = "D")]
    ObjectDestroyed,
    #[serde(rename = "M")]
    Metadata,
    #[serde(rename = "V")]
    MemoryDumpGlobal,
    #[serde(rename = "v")]
    MemoryDumpProcess,
    #[serde(rename = "R")]
    Mark,
    #[serde(rename = "c")]
    ClockSync,
    #[serde(rename = "(")]
    ContextBegin,
    #[serde(rename = ")")]
    ContextEnd,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(default, rename_all = "camelCase")]
pub struct TraceEvent {
    pub args: FxHashMap<String, serde_json::Value>,

    pub cat: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dur: Option<f64>, // Microseconds

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub name: String,

    pub ph: TraceEventType,

    pub pid: u64,

    pub tid: u64,

    pub ts: f64, // Microseconds

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<f64>, // Microseconds
}

impl TraceEvent {
    pub fn new(name: impl AsRef<String>, ph: TraceEventType) -> Self {
        Self {
            args: FxHashMap::default(),
            cat: String::new(),
            name: name.as_ref().to_owned(),
            ph,
            pid: std::process::id().into(),
            tid: 0, // TODO
            ts: std::time::SystemTime::now().elapsed().unwrap().as_nanos() as f64 / 1000.0,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct TraceJson(pub Vec<TraceEvent>);

impl Deref for TraceJson {
    type Target = Vec<TraceEvent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

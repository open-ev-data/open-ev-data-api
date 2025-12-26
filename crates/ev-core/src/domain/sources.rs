use serde::{Deserialize, Serialize};

use super::enums::SourceType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "type")]
    pub source_type: SourceType,

    pub title: String,
    pub url: String,
    pub accessed_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl Source {
    #[must_use]
    pub fn is_official(&self) -> bool {
        matches!(self.source_type, SourceType::Oem | SourceType::Regulatory)
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValidationError {
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Invalid year: {value}. Must be between 1900 and 2100")]
    InvalidYear { value: u16 },

    #[error("Invalid slug format: '{value}'. Must be lowercase alphanumeric with underscores")]
    InvalidSlug { value: String },

    #[error("Empty value not allowed for field: {field}")]
    EmptyValue { field: String },

    #[error("At least one battery capacity (gross or net) is required")]
    MissingBatteryCapacity,

    #[error("At least one charge port is required")]
    MissingChargePort,

    #[error("At least one rated range entry is required")]
    MissingRatedRange,

    #[error("At least one source is required")]
    MissingSource,

    #[error("Invalid URL format: {url}")]
    InvalidUrl { url: String },

    #[error("Invalid ISO country code: {code}")]
    InvalidCountryCode { code: String },

    #[error("Invalid ISO currency code: {code}")]
    InvalidCurrencyCode { code: String },

    #[error("Value out of range for {field}: {value} (expected {min}-{max})")]
    OutOfRange {
        field: String,
        value: String,
        min: String,
        max: String,
    },

    #[error("Invalid date-time format: {value}")]
    InvalidDateTime { value: String },

    #[error("Multiple validation errors: {0:?}")]
    Multiple(Vec<ValidationError>),
}

impl ValidationError {
    #[must_use]
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    #[must_use]
    pub fn invalid_slug(value: impl Into<String>) -> Self {
        Self::InvalidSlug {
            value: value.into(),
        }
    }

    #[must_use]
    pub fn empty_value(field: impl Into<String>) -> Self {
        Self::EmptyValue {
            field: field.into(),
        }
    }
}

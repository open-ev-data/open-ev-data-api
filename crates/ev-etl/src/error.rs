use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum EtlError {
    #[error("Failed to read file: {path}")]
    FileRead { path: String },

    #[error("Failed to parse JSON: {path}")]
    JsonParse { path: String },

    #[error("Merge error: {message}")]
    Merge { message: String },

    #[error("Validation failed for {vehicle_id}: {message}")]
    Validation { vehicle_id: String, message: String },

    #[error("Output generation failed: {format}")]
    OutputGeneration { format: String },

    #[error("Database error: {0}")]
    Database(String),

    #[error("CSV error: {0}")]
    Csv(String),

    #[error("IO error: {0}")]
    Io(String),
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, EtlError>;

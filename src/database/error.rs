use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(#[from] mongodb::error::Error),

    #[error("Serialization failed: {0}")]
    SerializationFailed(#[from] bson::ser::Error),

    #[error("Deserialization failed: {0}")]
    DeserializationFailed(#[from] bson::de::Error),

    #[error("Document not found")]
    NotFound,

    #[error("Duplicate document")]
    DuplicateKey,

    #[error("Invalid document format")]
    InvalidFormat,

    #[error("Database operation failed: {message}")]
    OperationFailed { message: String },

    #[error("Transaction failed")]
    TransactionFailed,

    #[error("Index creation failed")]
    IndexCreationFailed,

    #[error("Unknown database error")]
    Unknown,
}

impl DatabaseError {
    pub fn operation_failed(message: impl Into<String>) -> Self {
        Self::OperationFailed {
            message: message.into(),
        }
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound)
    }

    pub fn is_duplicate_key(&self) -> bool {
        matches!(self, Self::DuplicateKey)
    }
}

// Helper function to convert MongoDB errors to our custom error type
pub fn map_mongodb_error(error: mongodb::error::Error) -> DatabaseError {
    // Check error message for duplicate key patterns
    let error_msg = error.to_string();
    if error_msg.contains("E11000") || error_msg.contains("duplicate key") {
        return DatabaseError::DuplicateKey;
    }

    // For other errors, return a generic operation failed or connection error
    match error.kind.as_ref() {
        mongodb::error::ErrorKind::Write(_) => {
            DatabaseError::OperationFailed { message: error_msg }
        }
        mongodb::error::ErrorKind::Command(_) => {
            DatabaseError::OperationFailed { message: error_msg }
        }
        _ => DatabaseError::ConnectionFailed(error),
    }
}

use juniper::{graphql_value, FieldError, IntoFieldError, ScalarValue};
use thiserror::Error;

pub fn init_error_tracking() {}
pub fn capture_message_error(_err: Error) {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Error, Debug, PartialEq)]
pub enum Error {
    #[error("Serialization Error: {0}")]
    SerdeError(String),

    #[error("Network Error: {0}")]
    NetworkError(String),

    #[error("Invalid Decimal")]
    InvalidDecimal,

    // #[error("Invalid Id")]
    // InvalidId,
    #[error("Internal Server Error")]
    InternalServerError(String),

    #[error("Invalid Timestamp")]
    InvalidTimestamp(chrono::format::ParseError),
    // #[error("Database Error: {0}")]
    // DatabaseError(String),

    // #[error("Access denied")]
    // AccessDenied,

    // #[error("{0}")]
    // ValidationError(String),

    // #[error("Not found")]
    // NotFound,
}

impl<S> IntoFieldError<S> for Error
where
    S: ScalarValue,
{
    fn into_field_error(self) -> FieldError<S> {
        log::error!("{:?}", &self);
        capture_message_error(self.clone());

        match self {
            Error::SerdeError(e) => {
                FieldError::new("SERIALIZATION_ERROR", graphql_value!({ "message": e }))
            }
            Error::NetworkError(e) => {
                FieldError::new("NETWORK_ERROR", graphql_value!({ "message": e }))
            }
            Error::InternalServerError(_inner) => FieldError::new(
                "INTERNAL_SERVER_ERROR",
                graphql_value!({ "message": "Internal server error" }),
            ),
            Error::InvalidTimestamp(ref msg) => {
                let message = msg.to_string();
                FieldError::new("INVALID_TIMESTAMP", graphql_value!({ "message": message }))
            }
            Error::InvalidDecimal => FieldError::new(
                "INVALID_DECIMAL",
                graphql_value!({ "message": "Invalid decimal" }),
            ), // Error::DatabaseError(e) => {
               //     let err = Error::DatabaseError(e.clone());
               //     let message = format!("{:?}", e);
               //     log::error!("Database error: {:?}", e);
               //     //mightybadger::notify_std_error(&err);
               //     FieldError::new("DATABASE_ERROR", graphql_value!({ "message": message }))
               // }
               // Error::InvalidId => {
               //     let message = "Invalid Id".to_string();
               //     log::error!("Invalid ID");
               //     FieldError::new("INVALID_ID", graphql_value!({ "message": message }))
               // }
               // Error::NotFound => {
               //     log::error!("Not found");
               //     FieldError::new("NOT_FOUND", graphql_value!({ "message": "Not found" }))
               // }
               // Error::AccessDenied => {
               //     log::error!("Access denied");
               //     FieldError::new(
               //         "ACCESS_DENIED",
               //         graphql_value!({ "message": "Access denied" }),
               //     )
               // }
               // Error::ValidationError(message) => {
               //     log::error!("Validation error {}", &message);
               //     FieldError::new("VALIDATION_ERROR", graphql_value!({ "message": message }))
               // }
        }
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::NetworkError(format!("{:?}", e))
    }
}

impl std::convert::From<chrono::format::ParseError> for Error {
    fn from(e: chrono::format::ParseError) -> Error {
        Error::InvalidTimestamp(e)
    }
}

impl std::convert::From<bigdecimal::ParseBigDecimalError> for Error {
    fn from(_e: bigdecimal::ParseBigDecimalError) -> Error {
        Error::InvalidDecimal
    }
}

impl std::convert::From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error {
        Error::SerdeError(format!("{:?}", e))
    }
}

impl std::convert::From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Error {
        Error::InternalServerError(format!("{:?}", e))
    }
}

impl std::convert::From<bcrypt::BcryptError> for Error {
    fn from(e: bcrypt::BcryptError) -> Error {
        Error::InternalServerError(format!("{:?}", e))
    }
}

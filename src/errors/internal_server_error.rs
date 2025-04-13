use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct InternalServerError {
    pub message: String,
    pub date: String,
}

impl InternalServerError {
    /// Initializes a new InternalServerError
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    ///
    /// # Returns
    ///
    /// A new instance of InternalServerError
    pub fn new(message: &str) -> Self {
        InternalServerError {
            message: message.to_string(),
            date: chrono::Utc::now().to_rfc3339(),
        }
    }
}

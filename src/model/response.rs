use serde::{Deserialize, Serialize};

#[derive(Responder, Debug, Deserialize, Serialize)]
pub struct MessageResponse {
    /// This is a message from the server.
    pub message: String,
}
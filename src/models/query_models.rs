use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Object to capture a new emergency query body
#[derive(Serialize, Deserialize)]
pub struct EmergencyData {
    pub owner_id: Uuid,
    pub emergency_type: String,
    pub localization: String,
}

/// Object to create a new owner
#[derive(Serialize, Deserialize)]
pub struct OwnerData {
    pub email: String,
    pub username: String,
    pub country: String,
    pub biking_modality: String,
}

/// Object to create a new answer
#[derive(Serialize, Deserialize)]
pub struct AnswerData {
    pub parent_id: Uuid,
    pub answer: String,
}

/// Object to create a new message
#[derive(Serialize, Deserialize)]
pub struct MessageData {
    pub parent_id: Uuid,
    pub message_text: String,
}
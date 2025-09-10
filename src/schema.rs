use serde::{Deserialize, Serialize};

/// Query parameters for listing notes with pagination
#[derive(Debug, Default, Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

/// Schema for creating a new note
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,
}

/// Schema for updating an existing note
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>,
}
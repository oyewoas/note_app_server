use std::sync::Arc;

use axum::{routing::{
    get,
    post
}, Router};

use crate::{
    handler::{
        create_note_handler,
        health_check_handler,
        delete_note_handler,
        note_list_handler,
        update_note_handler,
        get_note_handler,
    },
    AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/healthcheck", get(health_check_handler))
        .route("/api/v1/notes", get(note_list_handler))
        .route("/api/v1/notes", post(create_note_handler))
        .route("/api/v1/notes/{id}", get(get_note_handler).patch(update_note_handler).delete(delete_note_handler))
        .with_state(state)
}
use std::sync::Arc;

use axum::{
    extract::{
        Path, Query, State,
    },
    response::IntoResponse,
    Json,
    http::StatusCode,
};

use serde_json::json;


use crate::{
    model::{
        NoteModel,
        NoteModelResponse,
    },
    schema::{
        CreateNoteSchema, FilterOptions, UpdateNoteSchema,
    },
    AppState,
};

// Convert DB Model to Response

fn to_note_response(note: &NoteModel) -> NoteModelResponse {
    NoteModelResponse {
        id: note.id.to_owned(),
        title: note.title.to_owned(),
        content: note.content.to_owned(),
        is_published: note.is_published != 0,
        created_at: note.created_at,
        updated_at: note.updated_at,
    }
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({ "status": "success", "message": MESSAGE });
    Json(json_response)
}


pub async fn note_list_handler(
    State(data): State<Arc<AppState>>,
    Query(filter): Query<FilterOptions>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    // Param

    let limit = filter.limit.unwrap_or(10);
    let offset = (filter.page.unwrap_or(1) - 1) * limit;

    // Query with macro
    let notes = sqlx::query_as!(
        NoteModel,
        r#"
        SELECT * FROM notes ORDER by id LIMIT ? OFFSET ?
        "#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|err| {
        let error_response = json!({ "status": "error", "message": format!("Database error: {}", err) });
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_response),
        )
    })?;
    let note_responses: Vec<NoteModelResponse> = notes.iter().map(to_note_response).collect();
    let json_response = serde_json::json!({ "status": "ok", "count": note_responses.len(), "notes": note_responses });
    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    // Insert
    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(
        r#"
        INSERT INTO notes (id, title, content, is_published) VALUES (?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(&payload.title)
    .bind(&payload.content)
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error|  err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = json!({ "status": "error", "message": "Note already exists" });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
        let error_response = json!({ "status": "error", "message": format!("Database error: {}", err) });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let new_note = sqlx::query_as!(
        NoteModel,
        r#"
        SELECT * FROM notes WHERE id = ?
        "#,
        &id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|err| {
        let error_response = json!({ "status": "error", "message": format!("Database error: {}", err) });
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_response),
        )
    })?;

    let note_response = to_note_response(&new_note);
    let json_response = serde_json::json!({ "status": "ok", "data": note_response });
    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_note_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let query_result = sqlx::query_as!(
        NoteModel,
        r#"
        SELECT * FROM notes WHERE id = ?
        "#,
        &id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = to_note_response(&note);
            let json_response = serde_json::json!({ "status": "ok", "data": note_response });
            Ok((StatusCode::OK, Json(json_response)))
        },
        Err(sqlx::Error::RowNotFound) => {
            let error_response = json!({ "status": "error", "message": "Note not found" });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        },
        Err(err) => {
            let error_response = json!({ "status": "error", "message": format!("Database error: {}", err) });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}


pub async fn update_note_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let query_result = sqlx::query_as!(
        NoteModel,
        r#"
        SELECT * FROM notes WHERE id = ?
        "#,
        &id
    )
    .fetch_one(&data.db)
    .await;

    let existing_note = match query_result {
        Ok(note) => note,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = json!({ "status": "error", "message": format!("Note with ID {} not found", id) });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        },
        Err(err) => {
            let error_response = json!({ "status": "error", "message": format!("Database error: {}", err) });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    let updated_title = payload.title.unwrap_or(existing_note.title);
    let updated_content = payload.content.unwrap_or(existing_note.content);
    let updated_is_published = payload.is_published.unwrap_or(existing_note.is_published != 0);
    let i8_updated_is_published = updated_is_published as i8;

    let update_result = sqlx::query(
        r#"
        UPDATE notes SET title = ?, content = ?, is_published = ? WHERE id = ?
        "#
    )
    .bind(updated_title)
    .bind(updated_content)
    .bind(i8_updated_is_published)
    .bind(&id)
    .execute(&data.db)
    .await
    .map_err(|err| {
        let error_response = json!({ "status": "error", "message": format!("Database error: {}", err) });
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_response),
        )
    })?;

    if update_result.rows_affected() == 0 {
        let error_response = json!({ "status": "error", "message": "Note not found" });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let updated_note = sqlx::query_as!(
        NoteModel,
        r#"
        SELECT * FROM notes WHERE id = ?
        "#,
        &id
    ).fetch_one(&data.db)
    .await
    .map_err(|err| {
        let error_response = json!({ "status": "error", "message": format!("Database error: {}", err) });
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_response),
        )
    })?;
    let note_response = to_note_response(&updated_note);
    let json_response = serde_json::json!({ "status": "ok", "data": note_response });
    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn delete_note_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let delete_result = sqlx::query!(
        r#"
        DELETE FROM notes WHERE id = ?
        "#,
        &id
    )
    .execute(&data.db)
    .await
    .map_err(|err| {
        let error_response = json!({ "status": "error", "message": format!("Database error: {}", err) });
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_response),
        )
    })?;

    if delete_result.rows_affected() == 0 {
        let error_response = json!({ "status": "error", "message": format!("Note with ID {} not found", id) });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let json_response = serde_json::json!({ "status": "ok", "message": "Note deleted successfully" });
    Ok((StatusCode::OK, Json(json_response)))
}
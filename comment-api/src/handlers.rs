use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::{Comment, CommentChild, CommentInput, CommentOutput};

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

// 댓글 추가
pub async fn add_comment(
    State(state): State<AppState>,
    Json(payload): Json<CommentInput>,
) -> Result<Json<Comment>, StatusCode> {
    let conn = state
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_uuid = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let hashed_password =
        hash(&payload.password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    conn.execute(
        "INSERT INTO comments (post, parent, content, writer, password, user_uuid, ip, created_at, deleted) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            &payload.post,
            &payload.parent,
            &payload.content,
            &payload.writer,
            &hashed_password,
            &user_uuid,
            None::<String>,
            &now,
            false,
        ),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = conn.last_insert_rowid();

    Ok(Json(Comment {
        id,
        post: payload.post,
        parent: payload.parent,
        content: payload.content,
        writer: payload.writer,
        password: "".to_string(),
        user_uuid,
        ip: None,
        created_at: now,
        deleted: false,
    }))
}

pub async fn get_comments(
    State(state): State<AppState>,
    Path(post): Path<String>,
) -> Result<Json<Vec<CommentOutput>>, StatusCode> {
    let conn = state
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut stmt = conn
        .prepare(
            "SELECT id, parent, content, writer, user_uuid, created_at, deleted 
             FROM comments WHERE post = ?1 ORDER BY created_at ASC",
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let comments_iter = stmt
        .query_map([&post], |row| {
            Ok(Comment {
                id: row.get(0)?,
                post: post.clone(),
                parent: row.get(1)?,
                content: row.get(2)?,
                writer: row.get(3)?,
                password: "".to_string(),
                user_uuid: row.get(4)?,
                ip: None,
                created_at: row.get(5)?,
                deleted: row.get(6)?,
            })
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut root_comments = Vec::new();
    let mut children_map = std::collections::HashMap::new();

    for c in comments_iter {
        let c = c.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(parent_id) = c.parent {
            children_map
                .entry(parent_id)
                .or_insert_with(Vec::new)
                .push(CommentChild {
                    id: c.id,
                    content: c.content,
                    writer: c.writer,
                    user_uuid: c.user_uuid,
                    created_at: c.created_at,
                    deleted: c.deleted,
                });
        } else {
            root_comments.push(CommentOutput {
                id: c.id,
                content: c.content,
                writer: c.writer,
                user_uuid: c.user_uuid,
                created_at: c.created_at,
                deleted: c.deleted,
                children: vec![],
            });
        }
    }

    for rc in &mut root_comments {
        if let Some(children) = children_map.get(&rc.id) {
            rc.children = children.clone();
        }
    }

    Ok(Json(root_comments))
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteCommentInput {
    pub password: String,
}

pub async fn delete_comment(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<DeleteCommentInput>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let conn = state
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut stmt = conn
        .prepare("SELECT password FROM comments WHERE id = ?1")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let hashed_password: String = stmt
        .query_row([id], |row| row.get(0))
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let is_valid = verify(&payload.password, &hashed_password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    conn.execute("UPDATE comments SET deleted = 1 WHERE id = ?1", [id])
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({"status": "deleted"})))
}

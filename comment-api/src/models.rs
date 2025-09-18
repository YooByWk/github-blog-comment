use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentInput {
    pub post: String,
    pub parent: Option<i64>,
    pub content: String,
    pub writer: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub post: String,
    pub parent: Option<i64>,
    pub content: String,
    pub writer: String,
    pub password: String,
    pub user_uuid: String,
    pub ip: Option<String>,
    pub created_at: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentOutput {
    pub id: i64,
    pub content: String,
    pub writer: String,
    pub user_uuid: String,
    pub created_at: String,
    pub deleted: bool,
    pub children: Vec<CommentChild>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentChild {
    pub id: i64,
    pub content: String,
    pub writer: String,
    pub user_uuid: String,
    pub created_at: String,
    pub deleted: bool,
}

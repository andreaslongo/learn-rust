use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, FromRow, Serialize, Clone, Debug)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub published_date: String,
}

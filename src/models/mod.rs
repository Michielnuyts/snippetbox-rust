use sqlx::{Pool, Postgres};

pub struct Snippet {
	pub id: i32,
	pub title: String,
	pub content: String,
	pub created: i32,
	pub expires: i32,
}

pub struct SnippetModel {
	pool: Pool<Postgres>,
}

impl SnippetModel {
	pub fn new(pool: Pool<Postgres>) -> Self {
		Self { pool }
	}
	pub async fn insert(&self, title: String, content: String) -> Result<i32, String> {
		todo!();
	}
}

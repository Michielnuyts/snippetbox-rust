use rocket::{response::status::NotFound, serde::json::Json, State};

use crate::App;

#[get("/snippet/<id>")]
pub fn snippet_view(id: usize) -> Result<Json<String>, NotFound<String>> {
	if id < 1 {
		return Err(NotFound(String::from("Did not find resource")));
	}

	Ok(Json(format!("Display a specific snippet with id {}", id)))
}

#[post("/snippet/create")]
pub async fn snippet_create(app: &State<App>) -> Json<String> {
	app.snippets
		.insert(String::from("Hey"), String::from("Ok"))
		.await;
	Json(String::from("Create a new snippet"))
}

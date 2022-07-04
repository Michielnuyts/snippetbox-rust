use rocket::{response::status::NotFound, serde::json::Json};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
	Template::render(
		"home",
		context! {
			title: "Test"
		},
	)
}

#[get("/snippet/view/<id>")]
pub fn snippet_view(id: usize) -> Result<Json<String>, NotFound<String>> {
	if id < 1 {
		return Err(NotFound(String::from("Did not find resource")));
	}

	Ok(Json(format!("Display a specific snippet with id {}", id)))
}

#[post("/snippet/create")]
pub fn snippet_create() -> Json<String> {
	Json(String::from("Create a new snippet"))
}

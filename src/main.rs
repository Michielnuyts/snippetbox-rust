use clap::Parser;
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod logger;
mod models;

#[macro_use]
extern crate rocket;

#[derive(Parser, Default, Debug)]
struct Arguments {
	#[clap(short, long, default_value_t = 8000)]
	/// Specify a custom port to start the server on
	port: u16,
}

pub struct App {
	snippets: models::SnippetModel,
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
	// Setup loggers
	// We can now use these logging macros globally
	// trace!() | debug!() | info!() | warn!() | error!()
	logger::setup().expect("initialize logging");

	// setup postgresql pool
	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect("postgres://local:local@localhost/snippetbox")
		.await
		.expect("tried to setup postgres connection pool");

	// when executing the binary, we want to provide some basic config options
	let args = Arguments::parse();
	let figment = rocket::Config::figment().merge(("port", args.port));

	// init new App instance
	let app = App {
		snippets: models::SnippetModel::new(pool),
	};

	// start the thing
	rocket::custom(figment).manage(app).mount(
		"/",
		routes![handlers::snippet_view, handlers::snippet_create],
	)
}

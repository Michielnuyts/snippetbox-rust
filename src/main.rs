use clap::Parser;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod handlers;
mod logger;

#[macro_use]
extern crate rocket;

#[derive(Parser, Default, Debug)]
struct Arguments {
	#[clap(short, long, default_value_t = 8000)]
	/// Specify a custom port to start the server on
	port: u16,
}

struct App {
	pool: Pool<Postgres>,
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
		.connect("postgres://postgres:password@localhost/test")
		.await
		.expect("tried to setup postgres connection pool");

	// when executing the binary, we want to provide some basic config options
	let args = Arguments::parse();
	info!("{:?}", args); // TODO attach custom arguments as server config

	// start the thing
	rocket::build().manage(App { pool }).mount(
		"/",
		routes![handlers::snippet_view, handlers::snippet_create],
	)
}

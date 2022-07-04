use clap::Parser;
use rocket_dyn_templates::Template;

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

#[launch]
fn rocket() -> _ {
	// Setup loggers
	// We can now use these logging macros globally
	// trace!() | debug!() | info!() | warn!() | error!()
	logger::setup().expect("initialize logging");

	// when executing the binary, we want to provide some basic config options
	let args = Arguments::parse();
	info!("{:?}", args); // TODO attach custom arguments as server config

	// start the thing
	rocket::build()
		.mount(
			"/",
			routes![
				handlers::index,
				handlers::snippet_view,
				handlers::snippet_create
			],
		)
		.attach(Template::fairing())
}

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let default_port: String = String::from("8080");
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let port = env::var("PORT").unwrap_or(default_port);
    let server = Server::new(format!("127.0.0.1:{}", port));
    server.run(WebsiteHandler::new(public_path));
}

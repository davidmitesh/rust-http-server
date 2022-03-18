#![allow(dead_code)]
mod server;
mod http;
use server::Server;
use website_handler::WebsiteHandler;
mod website_handler;
use std::env;
//It makes sense to path the path to public folder as the environment variable
//So for that, we make use of the environment variables in rust


fn main() {
    let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);//If PUBLIC_PATH is not passed as environment variable while cargo run, then it will take the default_path. 
    let server = Server :: new(String::from("127.0.0.1:8080") );
    server.run(WebsiteHandler::new(public_path));//We dont have to pass any parameter because it is empty struct
}











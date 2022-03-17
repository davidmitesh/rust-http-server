#![allow(dead_code)]
mod server;
mod http;
use server::Server;
use website_handler::WebsiteHandler;
mod website_handler;


fn main() {
   
    let server = Server :: new(String::from("127.0.0.1:8080") );
    server.run(WebsiteHandler);//We dont have to pass any parameter because it is empty struct
}











use http::Request;
use http::Method;
use server::Server;

mod http;
mod server;
fn main() {
    let string = String::from("127.0.0.1:8888");
    let server = Server::new(string);
    server.run();
}


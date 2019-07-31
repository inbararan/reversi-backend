mod server;
mod handler;
mod parser;
mod board;

fn main() {
    server::serve_incoming();
}

mod server;
mod handler;
mod parser;
mod game;
mod board;

fn main() {
    server::serve_incoming();
}

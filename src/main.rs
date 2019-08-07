mod server;
mod handler;
mod parser;
mod game;
mod board;
mod position;

fn main() {
    server::serve_incoming();
}

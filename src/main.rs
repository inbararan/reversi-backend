mod server;

fn main() {
    server::serve(|request| format!("\n\tRequest: {}, request len: {}\n", request, request.len()));
}

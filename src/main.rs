// src/main.rs
mod server;
mod handlers;

use std::env;

fn main() -> std::io::Result<()> {
    let root = env::current_dir()?;
    let socket_addr = "localhost:5500";
    server::serve(socket_addr, root)?;
    Ok(())
}
// src/server.rs
use std::{io::{self, Read}, net::{TcpListener, TcpStream}, path::PathBuf};
use crate::handlers::{file, directory};

pub fn serve(socket_addr: &str, root: PathBuf) -> io::Result<()> {
    let listener = TcpListener::bind(socket_addr)?;
    println!("Serving files from {} on {}", root.display(), socket_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let root = root.clone();
                std::thread::spawn(move || {
                    if let Err(e) = handle_client(stream, root) {
                        eprintln!("Error handling client: {:?}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {:?}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, root: PathBuf) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer);
    let request_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() < 2 || parts[0] != "GET" {
        return file::send_four_o_four(&mut stream);
    }

    let requested_path = root.join(parts[1].trim_start_matches('/'));

    if !requested_path.exists() {
        return file::send_four_o_four(&mut stream);
    }

    let canonical_root = root.canonicalize()?;
    let canonical_requested = requested_path.canonicalize()?;

    if !canonical_requested.starts_with(&canonical_root) {
        return file::send_four_o_four(&mut stream);
    }

    if requested_path.is_dir() {
        return directory::list_directory(&mut stream, &requested_path, parts[1]);
    }

    if requested_path.is_file() {
        return file::serve_file(&mut stream, &requested_path);
    }

    file::send_four_o_four(&mut stream)
}

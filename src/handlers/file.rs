use std::{fs, io::{self, Write}, net::TcpStream, path::Path};
use infer;

pub fn serve_file(stream: &mut TcpStream, path: &Path) -> io::Result<()> {
    let file_content = fs::read(path)?;

    let mime_type = infer::get_from_path(path)
        .ok()
        .flatten()
        .map(|kind| kind.mime_type())
        .unwrap_or("text/plain");

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
        file_content.len(),
        mime_type
    );

    stream.write(response.as_bytes())?;
    stream.write(&file_content)?;
    stream.flush()
}

pub fn send_four_o_four(stream: &mut TcpStream) -> io::Result<()> {
    let body = "<html><body><h1>404 - Not Found</h1></body></html>";
    let response = format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write(response.as_bytes())?;
    stream.flush()
}

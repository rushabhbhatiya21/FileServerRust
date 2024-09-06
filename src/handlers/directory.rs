use std::{io::{self, Write}, net::TcpStream, path::Path};
use walkdir::WalkDir;

pub fn list_directory(stream: &mut TcpStream, path: &Path, base_url: &str) -> io::Result<()> {
    let mut response_body = format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head><body><h1>Directory listing for {}</h1>",
        path.display()
    );

    if base_url.contains("/") {
        if let Some(pos) = base_url.rfind('/') {
            let parent_dir = &base_url[..pos];
            response_body.push_str(&format!("<a href=\"/{}\">Parent directory</a><br>", parent_dir));
        }
    }

    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_string_lossy();
        let file_url = format!("{}/{}", base_url.trim_end_matches('/'), file_name);
        response_body.push_str(&format!("<a href=\"{}\">{}</a><br>", file_url, file_name));
    }

    response_body.push_str("</body></html>");
    
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    stream.write(response.as_bytes())?;
    stream.flush()
}

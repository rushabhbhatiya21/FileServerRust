use std::{io::{self, Write}, net::TcpStream, path::Path};
use walkdir::WalkDir;

pub fn list_directory(stream: &mut TcpStream, path: &Path, base_url: &str) -> io::Result<()> {
    let current_dir_name = path.file_name().unwrap_or_default().to_string_lossy();
    
    let mut begin_html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8">
        <style>
            .highlight {
                color: red;
            }
        </style>
    </head>
    <body>
    "#.to_string();

    begin_html.push_str(&format!("<h1>Directory listing for {}</h1>", path.display()));

    if base_url.contains("/"){
        let parent_dir;
        if base_url == "/" {
            begin_html.push_str(&format!(
                r#"<a href="/">Parent directory</a><br>"#
            ))
        }else if let Some(pos) = base_url.rfind('/') {
            parent_dir = &base_url[..pos];
            begin_html.push_str(&format!(
                r#"<a href="/{}">Parent directory</a><br>"#,parent_dir.trim_start_matches('/')
            ));
        }
    }

    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_string_lossy();
        let file_url = format!("{}/{}", base_url.trim_end_matches('/'), file_name);

        // Check if the file/folder name matches the current directory name
        if file_name != current_dir_name {
            begin_html.push_str(&format!(
                r#"<a href="{}">{}</a><br>"#,
                file_url, file_name
            ));
        }
    }

    let end_html = r#"
    </body>
    </html>
    "#;

    let response_body = begin_html + &end_html;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    stream.write(response.as_bytes())?;
    stream.flush()
}

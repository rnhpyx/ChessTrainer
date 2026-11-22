use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread;

const INDEX: &[u8] = include_bytes!("../index.html");
const APP: &[u8] = include_bytes!("../app.js");
const STYLE: &[u8] = include_bytes!("../style.css");
const ENGINE_CLIENT: &[u8] = include_bytes!("../engine-client.js");
const CHESS: &[u8] = include_bytes!("../vendor/chess.js");
const STOCKFISH: &[u8] = include_bytes!("../vendor/stockfish/stockfish-19-lite-single.js");
const STOCKFISH_WASM: &[u8] = include_bytes!("../vendor/stockfish/stockfish-19-lite-single.wasm");
const STOCKFISH_LICENSE: &[u8] = include_bytes!("../vendor/stockfish/Copying.txt");
const CHESS_LICENSE: &[u8] = include_bytes!("../vendor/chess.js.LICENSE.txt");
const NOTICES: &[u8] = include_bytes!("../THIRD-PARTY-NOTICES.md");

fn asset(path: &str) -> Option<(&'static [u8], &'static str)> {
    match path {
        "/" | "/index.html" => Some((INDEX, "text/html; charset=utf-8")),
        "/app.js" => Some((APP, "text/javascript; charset=utf-8")),
        "/style.css" => Some((STYLE, "text/css; charset=utf-8")),
        "/engine-client.js" => Some((ENGINE_CLIENT, "text/javascript; charset=utf-8")),
        "/vendor/chess.js" => Some((CHESS, "text/javascript; charset=utf-8")),
        "/vendor/stockfish/stockfish-19-lite-single.js" => {
            Some((STOCKFISH, "text/javascript; charset=utf-8"))
        }
        "/vendor/stockfish/stockfish-19-lite-single.wasm" => {
            Some((STOCKFISH_WASM, "application/wasm"))
        }
        "/vendor/stockfish/Copying.txt" => Some((STOCKFISH_LICENSE, "text/plain; charset=utf-8")),
        "/vendor/chess.js.LICENSE.txt" => Some((CHESS_LICENSE, "text/plain; charset=utf-8")),
        "/THIRD-PARTY-NOTICES.md" => Some((NOTICES, "text/markdown; charset=utf-8")),
        _ => None,
    }
}

fn respond(mut stream: TcpStream) -> std::io::Result<()> {
    let mut request = [0; 8192];
    let count = stream.read(&mut request)?;
    let first_line = String::from_utf8_lossy(&request[..count]);
    let path = first_line
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/")
        .split('?')
        .next()
        .unwrap_or("/");

    if let Some((body, content_type)) = asset(path) {
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n\r\n",
            body.len()
        )?;
        stream.write_all(body)?;
    } else {
        stream.write_all(
            b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        )?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let url = format!("http://{address}/");
    println!("ChessTrainer is running at {url}");
    println!("Keep this window open while you train. Close it to stop the app.");
    std::io::stdout().flush()?;

    #[cfg(target_os = "windows")]
    if std::env::var_os("CHESSTRAINER_NO_BROWSER").is_none() {
        let _browser = Command::new("cmd")
            .args(["/C", "start", "", &url])
            .spawn();
    }

    #[cfg(target_os = "macos")]
    if std::env::var_os("CHESSTRAINER_NO_BROWSER").is_none() {
        let _browser = Command::new("open").arg(&url).spawn();
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    if std::env::var_os("CHESSTRAINER_NO_BROWSER").is_none() {
        let _browser = Command::new("xdg-open").arg(&url).spawn();
    }

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                thread::spawn(|| {
                    let _ = respond(stream);
                });
            }
            Err(error) => eprintln!("Local connection error: {error}"),
        }
    }
    Ok(())
}

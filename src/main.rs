use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use web_server::{Config, ThreadPool};
use std::thread;
use std::time::Duration;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build_from_args(&args);
    let server = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(config.max_workers);
    for stream in server.incoming().take(config.max_requests) {
        let stream = stream.unwrap();
        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(format!("static/{}", filename)).unwrap();
    let response = format!("{}{}", status_line, contents);
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
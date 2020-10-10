use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:7878");
    match tcp_listener {
        Ok(listener) => listen_incoming_calls(listener),
        Err(_) => println!("Ohh ho!!!"),
    };
}

fn listen_incoming_calls(listener: TcpListener) {
    let pool_result = ThreadPool::new(4);
    match pool_result {
        Ok(pool) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(found) => {
                        // println!("Connection Estabilished!!!");
                        pool.execute(|| {
                            handle_connection(found);
                        });
                    }
                    Err(_) => println!("Ohh ho!!!"),
                }
            }
        }
        Err(err) => println!("{}", err),
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let read_stream = stream.read(&mut buffer);
    let mut contents: String = String::from("");
    let mut response: String = String::from("");

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    match read_stream {
        Ok(_result) => {
            let mut req = String::from_utf8_lossy(&buffer[..]);
            // println!("Request: {}", req);

            if buffer.starts_with(sleep) {
                thread::sleep(Duration::from_secs(5));
                contents = fs::read_to_string("hello.html")
                    .unwrap()
                    .replace("_CONTENT_", req.to_mut());

                response = format!(
                    "HTTP/1.1 200 OK\r\nContent-length: {}\r\n\r\n{}",
                    contents.len(),
                    contents
                );
            } else if buffer.starts_with(get) {
                contents = fs::read_to_string("hello.html")
                    .unwrap()
                    .replace("_CONTENT_", req.to_mut());

                response = format!(
                    "HTTP/1.1 200 OK\r\nContent-length: {}\r\n\r\n{}",
                    contents.len(),
                    contents
                );
            } else {
                let states_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                contents = fs::read_to_string("404.html").unwrap();

                response = format!("{}{}", states_line, contents);
            }
        }
        Err(_) => println!("Something wrong with stream decoding..."),
    }

    let write_stream = stream.write(response.as_bytes());

    match write_stream {
        Ok(_) => {
            // println!("Response Send ");
        }
        Err(_) => {
            println!("error in sendind response");
        }
    }

    match stream.flush() {
        Ok(_) => {
            // println!("Flushed...");
        }
        Err(_) => {
            println!("not flushed");
        }
    }
}

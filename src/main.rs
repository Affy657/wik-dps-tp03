use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
};
use gethostname::gethostname;

fn main() {
    let port = match env::var("PING_LISTEN_PORT") {
        Ok(port) => port,
        Err(_) => "7878".to_string(),
    };
        
    let listener = TcpListener::bind(format!("0.0.0.0:{}",port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = _http_request[0].clone();
    let header_line: Vec<_> = _http_request[1..].to_vec();

    let mut headers =HashMap::new();

    for line in header_line {
        let mut header = line.split(":");
        let key = header.next().unwrap().to_string();
        let value = header.next().unwrap().to_string();
        headers.insert(key, value);
    }

    if request_line == "GET /ping HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = heshmap_json(&headers);
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        println!("Hostname: {:?}", gethostname());
        stream.write_all(response.as_bytes()).unwrap();

    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = "";
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}
fn heshmap_json(headers: &HashMap<String, String>) -> String {
    let mut json = String::new();
    json.push_str("{");
    for (key, value) in headers {
        json.push_str(&format!("\"{}\":\"{}\",", key, value));
    }
    json.pop();
    json.push_str("}");
    json
}
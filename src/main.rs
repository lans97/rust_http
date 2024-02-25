use std::{collections::HashMap, io::Read, net::{TcpListener, TcpStream}};

struct Request<'a> {
    method: &'a str,
    resource: &'a str,
    scheme: &'a str,
    headers: HashMap<String, String>
}

impl<'a> Request<'a> {
    fn parse(request: &'a [u8]) -> Option<Request<'a>> {
        
        let request_str = std::str::from_utf8(request).ok()?;

        let mut lines = request_str.lines();
        
        let request_line = lines.next()?;
        let mut request_parts = request_line.split_whitespace();
        let method = request_parts.next()?;
        let resource = request_parts.next()?;
        let scheme = request_parts.next()?;
        
        let mut headers = HashMap::new();
        for line in lines {
            if let Some((header, value)) = line.split_once(": ") {
                headers.insert(header.to_string(), value.to_string());
            }
        }
        
        Some(Request{
            method,
            resource,
            scheme,
            headers
        })
    }
}

fn handle_request(request: &Request) {
    match request.method {
        "GET" => {
            
        },
        "POST" => {

        },
        "PUT" => {
                        
                    },
        "DELETE" => {
                        
                    },
        _ => println!("Error")
    }
}

fn run_http(host: &str, port: u32) {
    let socket = format!("{host}:{port}");
    let listener = TcpListener::bind(socket).unwrap();
    for stream in listener.incoming() {
        let mut buffer = Vec::new();
        stream.unwrap().read_to_end(&mut buffer).unwrap();
        let req = Request::parse(&buffer).unwrap();
        handle_request(& req);
    }
}

fn main() {
    run_http("localhost", 80);
}

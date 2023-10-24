use std::{thread, net, io::{self, BufRead, Write}};
pub use http_bytes::http;

pub type Router = fn(&str) -> http::Response<Vec<u8>>;

pub struct GisServer {
    addr: String,
    router: Router
}

fn handle_connection(router:Router, mut stream: &net::TcpStream) {
    let mut reader = io::BufReader::new(&mut stream);
    let (request, _) = http_bytes::parse_request_header_easy(reader.fill_buf().unwrap()).unwrap().unwrap();

    let response = (router)(request.uri().path());
    stream.write_all(&http_bytes::response_header_to_vec(&response)).unwrap();
    stream.write_all(response.body()).unwrap();
}

impl GisServer {
    pub fn new(addr:&str, router:Router) -> GisServer {
        GisServer{
            addr: String::from(addr),
            router: router,
        }
    }

    pub fn start(&self) {
        let server = net::TcpListener::bind(&self.addr).unwrap();
        for stream in server.incoming() {
            let router = self.router;
            let stream = stream.unwrap();
            
            thread::spawn(move || {
                handle_connection(router, &stream);
            });
        }
    }
}





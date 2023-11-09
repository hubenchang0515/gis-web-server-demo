use std::{net, io::{self, BufRead, Write}};
pub use http_bytes::http;

pub trait Router {
    fn route(&self, path:&str) -> http::Response<Vec<u8>>;
}

pub struct GisServer {
}

fn handle_connection(router:&impl Router, mut stream: &net::TcpStream) {
    let mut reader = io::BufReader::new(&mut stream);
    let (request, _) = http_bytes::parse_request_header_easy(reader.fill_buf().unwrap()).unwrap().unwrap();

    let response = router.route(request.uri().path());
    stream.write_all(&http_bytes::response_header_to_vec(&response)).unwrap();
    let _ = stream.write_all(response.body());
}

impl GisServer {
    pub fn start(addr:&str, router:&impl Router) {
        let server = net::TcpListener::bind(addr).unwrap();
        for stream in server.incoming() {
            let stream = stream.unwrap();
            handle_connection(router, &stream);
        }
    }
}





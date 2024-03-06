use std::{
    collections::{hash_map, HashMap},
    io::{BufReader, Error, Read},
    net::{TcpListener, TcpStream},
};

pub struct Weba {
    routes: Vec<(String, fn() -> ())>,
}

impl Weba {
    pub fn new() -> Self {
        Weba { routes: vec![] }
    }

    // Creates new route
    pub fn route(mut self, name: &str, function: fn() -> ()) -> Self {
        self.routes.push((String::from(name), function));
        self
    }

    pub fn run(&self, ip: &str) {
        let listener = TcpListener::bind(ip);

        match listener {
            Result::Ok(listener) => {
                for stream in listener.incoming() {
                    self.handle_client(stream);
                }
            }

            Result::Err(err) => {
                eprintln!("Error in run: {}", err);
            }
        }
    }

    fn handle_client(&self, client: Result<TcpStream, Error>) {
        match client {
            Result::Ok(client) => {
                let mut reader = BufReader::new(&client);
                let mut request_string = String::new();
                reader
                    .read_to_string(&mut request_string)
                    .expect("Reader error");

                let new_request = Request::new(request_string);
            }

            Result::Err(err) => {
                eprintln!("Error while handling client: {err}");
            }
        }
    }
}

struct Request {
    method: String,
    path: String,
    version: String,
    headers: Headers,
}

impl Request {
    pub fn new(request_string: String) -> Request {
        let request_split: Vec<&str> = request_string.split('\n').collect();
        let request_line = Request::get_request_line(request_split[0]);
        let headers_map = Headers::get_headers(&request_split[1..].join("\n"));

        Request {
            method: String::from(&request_line[0]),
            path: String::from(&request_line[1]),
            version: String::from(&request_line[2]),
            headers: Headers {
                headers: headers_map,
            },
        }
    }

    // Returns a Vector where:
    // 0 index contains method
    // 1 index contains path
    // 2 index contains version
    fn get_request_line(request_string: &str) -> Vec<String> {
        request_string.split(' ').map(|s| String::from(s)).collect()
    }
}

struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    // Gets headers from a string that contains headers.
    pub fn get_headers(request: &str) -> HashMap<String, String> {
        let headers_splited = request.split('\n');

        let mut headers: HashMap<String, String> = HashMap::new();
        for string in headers_splited {
            let (key, value) = string.split_at(string.find(": ").unwrap());
            headers.insert(String::from(key), String::from(value));
        }

        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes() {
        fn index() {
            println!("test");
        }

        let weba = Weba::new().route("/", index).route("/test", index);

        assert_eq!(weba.routes[0].0, String::from("/"))
    }
}

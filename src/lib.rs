use std::{
    collections::HashMap,
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
        Request {
            method: String::from("a"),
            path: String::from("n"),
            version: String::from("1"),
            headers: Headers {
                headers: HashMap::new(),
            },
        }
    }
}

struct Headers {
    headers: HashMap<String, String>,
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

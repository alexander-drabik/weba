use std::{
    io::Error,
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
            Result::Ok(client) => {}
            Result::Err(err) => {
                eprintln!("Error while handling client: {err}");
            }
        }
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

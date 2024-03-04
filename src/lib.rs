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

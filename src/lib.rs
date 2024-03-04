pub struct Weba {
    routes: Vec<String>,
}

impl Weba {
    pub fn new() -> Self {
        Weba { routes: vec![] }
    }

    pub fn route(mut self, name: &str) -> Self {
        self.routes.push(String::from(name));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes() {
        let weba = Weba::new().route("/").route("/test");

        assert_eq!(weba.routes, vec![String::from("/"), String::from("/test")])
    }
}

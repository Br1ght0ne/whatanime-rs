extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod api;
mod config;

fn identify(path: &str) -> &str {
    path
}

fn run() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use config::Config;

    fn config() -> Config {
        Config::from_file("config.yml").unwrap()
    }

    mod api {
        use super::*;
        use api::Api;

        #[test]
        fn identity() {
            let config = config();
            let api = Api::new(config.token);
            let me = api.identity().unwrap();
            me.email;
        }
    }
    #[test]
    fn it_works() {
        unimplemented!()
    }
}

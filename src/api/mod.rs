mod responses;

use self::responses::Identity;
use reqwest::{Client, Response};

pub const API_ROOT: &str = "https://whatanime.ga/api";

error_chain! {
    errors {
        RequestFailed(url: String) {
            description("request failed")
            display("request failed: {}", url)
        }
        ResponseParse {
            description("failed to parse response")
            display("failed to parse response")
        }
    }
}

pub struct Api {
    pub token: String,
    pub client: Client,
}

impl Api {
    pub fn new(token: String) -> Self {
        let client = Client::new();
        Self { token, client }
    }

    fn path(&self, part: &String) -> String {
        format!("{}/{}", API_ROOT, part)
    }

    fn form<'a>(&'a self, content: &[(&'a str, &'a str)]) -> Vec<(&'a str, &'a str)> {
        let mut form: Vec<(&str, &str)> = Vec::new();
        form.copy_from_slice(content);
        form.push(("token", &self.token));
        form
    }

    fn post_form(
        &self,
        path: String,
        form: &[(&str, &str)],
    ) -> Result<Response> {
        let response = self
            .client
            .post(&self.path(&path))
            .form(&self.form(&form))
            .send()
            .chain_err(|| ErrorKind::RequestFailed(path))?;
        Ok(response)
    }

    fn get(&self, path: String) -> Result<Response> {
        let response = self
            .client
            .get(&self.path(&path))
            .send()
            .chain_err(|| ErrorKind::RequestFailed(path))?;
        Ok(response)
    }

    pub fn identity(&self) -> Result<Identity> {
        let me: Identity = self
            .get("/me".into())?
            .json()
            .chain_err(|| ErrorKind::ResponseParse)?;
        Ok(me)
    }
}

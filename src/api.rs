use std::collections::HashMap;

static API_URL: &'static str = "https://yuko.app/api/v1/";

pub struct API {
    client: reqwest::Client,
    token: String,
}

impl API {
    pub fn new(token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token.to_owned()
        }
    }

    fn url(&self, path: &str) -> reqwest::Url {
        let base = reqwest::Url::parse(API_URL).unwrap();
        base.join(path).unwrap()
    }

    pub fn get(&self, path: &str, args: HashMap<String, String>) -> Result<reqwest::Response, reqwest::Error> {
        let mut url = self.url(path);

        for (param, value) in &args {
            url.query_pairs_mut()
                .append_pair(&param, &value);
        }

        return self.client
            .get(url)
            .header("Authorization", &format!("{} {}", "Bearer", self.token))
            .send()
    }

    pub fn post(&self, path: &str, json: HashMap<String, String>) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.url(path);

        self.client
            .post(url)
            .header("Authorization", &format!("{} {}", "Bearer", self.token))
            .json(&json)
            .send()
    }
}

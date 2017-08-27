#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate serde;

pub mod time_entries;

use serde::ser::Serialize;

pub struct RedmineApi {
    pub time_entries: time_entries::Api,
}
impl RedmineApi {
    pub fn new(host: String, apikey: String) -> RedmineApi {
        RedmineApi {
            time_entries: time_entries::Api::new(RedmineClient::new(host, apikey)),
        }
    }
}

pub struct RedmineClient {
    host: String,
    apikey: String,
}
impl RedmineClient {
    fn new(host: String, apikey: String) -> RedmineClient {
        RedmineClient {
            host: host,
            apikey: apikey,
        }
    }

    fn create<T: Serialize>(&self, path: &str, object: &T) -> bool {
        let client = reqwest::Client::new().unwrap();
        client.post(self.get_base_url(path).as_str()).unwrap()
            .json(object).unwrap()
            .send().unwrap();

        true
    }

    fn get_base_url(&self, path: &str) -> reqwest::Url {
        let url_string = self.host.clone() + path;
        let mut url = reqwest::Url::parse(&url_string).unwrap();
        url.query_pairs_mut()
            .append_pair("key", &self.apikey);

        url
    }
}

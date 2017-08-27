#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate url;

pub mod time_entries;

use serde::ser::Serialize;
use self::url::Url;

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
        let mut options = Vec::new();
        options.push(("key", &self.apikey));

        let url_string = self.host.clone() + path;
        let mut url = Url::parse(&url_string).unwrap();
        url.set_query_from_pairs(options);

        let client = reqwest::Client::new().unwrap();
        client.request(reqwest::Method::Post, &url.serialize()).unwrap()
            .json(object).unwrap()
            .send().unwrap();

        true
    }
}

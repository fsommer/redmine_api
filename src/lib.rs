#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate serde;

pub mod time_entries;

use std::io::Read;
use reqwest::{Client, Url};
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

    fn list(&self, path: &str) -> String {
        let mut response = Client::new().unwrap()
            .get(self.get_base_url(path).as_str()).unwrap()
            .send().unwrap();

        let mut result = String::new();
        response.read_to_string(&mut result).unwrap();

        result
    }

    fn create<T: Serialize>(&self, path: &str, object: &T) -> bool {
        Client::new().unwrap()
            .post(self.get_base_url(path).as_str()).unwrap()
            .json(object).unwrap()
            .send().unwrap();

        true
    }

    fn get_base_url(&self, path: &str) -> Url {
        let mut url = Url::parse(&(self.host.clone() + path)).unwrap();
        url.query_pairs_mut()
            .append_pair("key", &self.apikey);

        url
    }
}

#[derive(Deserialize, Debug)]
pub struct Activity {
    id: u32,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    id: u32,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    id: u32,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    id: u32,
    name: String,
}

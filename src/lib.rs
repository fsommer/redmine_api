#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate serde;

pub mod issues;
pub mod time_entries;

use std::collections::HashMap;
use std::io::Read;
use reqwest::{Client, Url};
use serde::ser::Serialize;

pub struct RedmineApi {
    issues: issues::Api,
    time_entries: time_entries::Api,
}
impl RedmineApi {
    pub fn new(host: String, apikey: String) -> RedmineApi {
        RedmineApi {
            issues: issues::Api::new(RedmineClient::new(host.clone(), apikey.clone())),
            time_entries: time_entries::Api::new(RedmineClient::new(host.clone(), apikey.clone())),
        }
    }

    pub fn issues(&self) -> &issues::Api {
        &self.issues
    }

    pub fn time_entries(&self) -> &time_entries::Api {
        &self.time_entries
    }
}

#[derive(Clone)]
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

    fn list(&self, path: &str, params: &HashMap<&str, String>) -> String {
        let mut url = self.get_base_url(path);

        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }

        let mut response = Client::new().unwrap()
            .get(url.as_str()).unwrap()
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
pub struct Object {
    id: u32,
}

#[derive(Deserialize, Debug)]
pub struct NamedObject {
    id: u32,
    name: String,
}

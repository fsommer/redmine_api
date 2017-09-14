#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate serde;

pub mod issues;
pub mod time_entries;
pub mod errors;

use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;
use reqwest::{Client, Url};
use reqwest::header::Location;
use serde::ser::Serialize;
use errors::*;

pub struct RedmineApi {
    issues: issues::Api,
    time_entries: time_entries::Api,
}
impl RedmineApi {
    pub fn new(host: String, apikey: String) -> RedmineApi {
        let c = Rc::new(RedmineClient::new(host, apikey));
        RedmineApi {
            issues: issues::Api::new(Rc::clone(&c)),
            time_entries: time_entries::Api::new(Rc::clone(&c)),
        }
    }

    pub fn issues(&self) -> &issues::Api {
        &self.issues
    }

    pub fn time_entries(&self) -> &time_entries::Api {
        &self.time_entries
    }
}

#[derive(Debug, Default)]
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

    fn get(&self, path: &str, params: &HashMap<&str, String>) -> Result<String> {
        let mut url = self.get_base_url(path)?;

        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }

        let mut response = Client::new()?
            .get(url.as_str())?
            .send()?;

        let mut result = String::new();
        response.read_to_string(&mut result)?;

        Ok(result)
    }

    fn create<T: Serialize>(&self, path: &str, object: &T) -> Result<String> {
        let mut response = Client::new()?
            .post(self.get_base_url(path)?.as_str())?
            .json(object)?
            .send()?;

        if !response.status().is_success() {
            let mut body = String::new();
            response.read_to_string(&mut body)?;
            bail!("Error: {}, {}", response.status(), body);
        }

        match response.headers().get::<Location>() {
            Some(l) => Ok(l.to_string()),
            _ => bail!("Can't create issue."),
        }
    }

    fn update<T: Serialize>(&self, path: &str, object: &T) -> Result<String> {
        let mut response = Client::new()?
            .put(self.get_base_url(path)?.as_str())?
            .json(object)?
            .send()?;

        if !response.status().is_success() {
            let mut body = String::new();
            response.read_to_string(&mut body)?;
            bail!("Error: {}, {}", response.status(), body);
        }

        Ok("Success".to_string())
    }

    fn get_base_url(&self, path: &str) -> Result<Url> {
        let mut url = Url::parse(&(self.host.clone() + path))
            .chain_err(|| format!("Can't parse url: {}", (self.host.clone() + path)))?;

        url.query_pairs_mut()
            .append_pair("key", &self.apikey);

        Ok(url)
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Object {
    id: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct NamedObject {
    id: u32,
    name: String,
}

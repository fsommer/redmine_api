//! This library can be used to communicate with an existing redmine application. All you need is
//! an up and running redmine application and a valid api key. See
//! [RedmineApi](struct.RedmineApi.html) struct to get started.

#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod errors;
pub mod issues;
pub mod time_entries;

use errors::*;
use reqwest::header::Location;
use reqwest::{Client, Response, Url};
use serde::ser::Serialize;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;

/// This struct represents the entry point to the stable redmine api. It gets a host url and an api
/// key for instantiation and exposes all kind of different apis provided by redmine.
///
/// # Example
///
/// ```
/// use redmine_api::RedmineApi;
///
/// let redmine = RedmineApi::new(
///     "http://www.redmine.org/".to_string(),
///     "1234".to_string()
/// );
///
/// let result = redmine.issues().show(1).execute();
/// ```
pub struct RedmineApi {
    issues: issues::Api,
    time_entries: time_entries::Api,
}
impl RedmineApi {
    /// Creates a new instance.
    ///
    /// # Arguments
    ///
    /// * `host` - a string holding the url of a redmine application
    /// * `apikey` - a string holding the apikey provided by redmine
    pub fn new(host: String, apikey: String) -> RedmineApi {
        let c = Rc::new(RedmineClient::new(host, apikey));
        RedmineApi {
            issues: issues::Api::new(Rc::clone(&c)),
            time_entries: time_entries::Api::new(Rc::clone(&c)),
        }
    }

    /// Provides issues api.
    pub fn issues(&self) -> &issues::Api {
        &self.issues
    }

    /// Provides time entries api.
    pub fn time_entries(&self) -> &time_entries::Api {
        &self.time_entries
    }
}

/// Holds host and api key and provides generic functions for get, post, delete, etc.. Is only used
/// internally.
#[derive(Debug, Default)]
pub struct RedmineClient {
    host: String,
    apikey: String,
}
impl RedmineClient {
    /// Creates new instance.
    ///
    /// # Arguments
    ///
    /// * `host` - a string holding the redmine host url
    /// * `apikey` - a string holding a valid redmine api key
    fn new(host: String, apikey: String) -> RedmineClient {
        RedmineClient {
            host: host,
            apikey: apikey,
        }
    }

    /// Performs GET request to api endpoint specified by `path`, transcoding the `params` argument
    /// to query string. Returns the response body as string.
    ///
    /// # Arguments
    ///
    /// * `path` - a string slice holding the api endpoint, e.g. '/issues.json'
    /// * `params` - a hashmap holding query parameters
    fn get(&self, path: &str, params: &HashMap<&str, String>) -> Result<String> {
        let mut url = self.get_base_url(path)?;

        // transcode parameters to query string
        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }

        let mut response = Client::new()?.get(url.as_str())?.send()?;

        // read response body
        let mut result = String::new();
        response.read_to_string(&mut result)?;

        Ok(result)
    }

    /// Performs POST request to api endpoint specified by `path` for creating a new `object`.
    /// Returns content of location header as string.
    ///
    /// # Arguments
    ///
    /// * `path` - a string slice holding the api endpoint, e.g. '/issues.json'
    /// * `object` - a struct implementing the serde Serialize trait
    fn create<T: Serialize>(&self, path: &str, object: &T) -> Result<String> {
        let mut response = self.post(path, object)?;

        // put response body in error message if request has failed
        if !response.status().is_success() {
            let mut body = String::new();
            response.read_to_string(&mut body)?;
            bail!("Error: {}, {}", response.status(), body);
        }

        // return content of the location header, which holds the url of the created issue.
        match response.headers().get::<Location>() {
            Some(l) => Ok(l.to_string()),
            _ => bail!("Can't create issue."),
        }
    }

    /// Performs PUT request to api endpoint specified by `path` for updating an entity with data
    /// provided by `object`.
    ///
    /// # Arguments
    ///
    /// * `path` - a string slice holding the api endpoint, e.g. '/issues/1.json'
    /// * `object` - a struct implementing the serde Serialize trait
    fn update<T: Serialize>(&self, path: &str, object: &T) -> Result<String> {
        let mut response = Client::new()?
            .put(self.get_base_url(path)?.as_str())?
            .json(object)?
            .send()?;

        // put response body in error message if request has failed
        if !response.status().is_success() {
            let mut body = String::new();
            response.read_to_string(&mut body)?;
            bail!("Error: {}, {}", response.status(), body);
        }

        Ok("Success".to_string())
    }

    /// Performs DELETE request to api endpoint specified by `path`.
    ///
    /// # Arguments
    ///
    /// * `path` - a string slice holding the api endpoint, e.g. '/issues/1.json'
    fn delete(&self, path: &str) -> Result<bool> {
        let response = Client::new()?
            .delete(self.get_base_url(path)?.as_str())?
            .send()?;

        if !response.status().is_success() {
            bail!("Error: {}", response.status());
        }

        Ok(true)
    }

    /// Performs generic POST request to api endpoint specified by `path` and sends embedded
    /// information of `object`. Returns reqwest response.
    ///
    /// # Arguments
    ///
    /// * `path` - a string slice holding the api endpoint, e.g. '/issues.json'
    /// * `object` - a struct implementing the serde Serialize trait
    fn post<T: Serialize>(&self, path: &str, object: &T) -> Result<Response> {
        Client::new()?
            .post(self.get_base_url(path)?.as_str())?
            .json(object)?
            .send()
            .chain_err(|| format!("Can't post to {}", path))
    }

    /// Returns fully qulaified url to an redmine api endpoint (assuming the host user provided
    /// `host` parameter is valid). Returns reqwest Url.
    ///
    /// # Arguments
    ///
    /// * `path` - a string slice holding the api endpoint, e.g. '/issues.json'
    fn get_base_url(&self, path: &str) -> Result<Url> {
        let mut url = Url::parse(&(self.host.clone() + path)).chain_err(|| {
            format!("Can't parse url: {}", (self.host.clone() + path))
        })?;

        url.query_pairs_mut().append_pair("key", &self.apikey);

        Ok(url)
    }
}

/// Generic helper struct to wrap an id. Is used for deserialization of redmine json responses.
#[derive(Deserialize, Debug, Default)]
pub struct Object {
    id: u32,
}

/// Generic helper struct to wrap an id and a name. Is used for deserialization of redmine json
/// responses.
#[derive(Deserialize, Debug, Default)]
pub struct NamedObject {
    id: u32,
    name: String,
}

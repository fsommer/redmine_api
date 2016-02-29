extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub mod time_entries;

use self::hyper::Client;
use self::hyper::header::ContentType;
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

    fn create(&self, path: &str, object: &str) -> bool {
        let mut options = Vec::new();
        options.push(("key", &self.apikey));

        let url_string = self.host.clone() + path;
        let mut url = Url::parse(&url_string).unwrap();
        url.set_query_from_pairs(options);

        let client = Client::new();
        client.post(url)
            .header(ContentType::json())
            .body(object)
            .send()
            .unwrap();

        true
    }
}

use super::RedmineClient;
use rustc_serialize::json;

pub struct Api {
    client: RedmineClient,
}
impl Api {
    pub fn new(client: RedmineClient) -> Api {
        Api {
            client: client,
        }
    }

    pub fn create(&self, time_entry: &TimeEntry) -> bool {
        let json = json::encode(time_entry).unwrap();
        self.client.create("/time_entries.json", &json)
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TimeEntry {
    pub issue_id: u32,
    pub hours: f32,
    pub activity_id: u8,
    pub comments: String,
}

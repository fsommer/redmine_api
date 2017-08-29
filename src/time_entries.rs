extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use super::{Object, NamedObject};
use super::RedmineClient;

pub struct Api {
    client: Rc<RedmineClient>,
}
impl Api {
    pub fn new(client: Rc<RedmineClient>) -> Api {
        Api {
            client: client,
        }
    }

    pub fn list(&self) -> Result<TimeEntryList, Box<Error>> {
        let result = self.client.list("/time_entries.json", &HashMap::new())?;

        match serde_json::from_str(&result) {
            Ok(list) => Ok(list),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn create(&self, time_entry: &TimeEntry) -> Result<bool, Box<Error>> {
        self.client.create("/time_entries.json", &CreateTimeEntry {
            time_entry: time_entry
        })
    }
}

#[derive(Serialize)]
pub struct TimeEntry {
    pub issue_id: u32,
    pub hours: f32,
    pub activity_id: u8,
    pub comments: String,
}

#[derive(Deserialize, Debug)]
pub struct TimeEntryList {
    time_entries: Vec<TimeEntryListItem>,
}

#[derive(Deserialize, Debug)]
pub struct TimeEntryListItem {
    pub activity: NamedObject,
    pub comments: String,
    pub hours: f32,
    pub id: u32,
    pub issue: Object,
    pub project: NamedObject,
    pub user: Object,
    pub spent_on: String,
    pub created_on: String,
    pub updated_on: String,
}

#[derive(Serialize)]
struct CreateTimeEntry<'a> {
    time_entry: &'a TimeEntry,
}

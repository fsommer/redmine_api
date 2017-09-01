extern crate serde_json;

use std::collections::HashMap;
use std::rc::Rc;
use super::errors::*;
use super::{Object, NamedObject, RedmineClient};

pub struct Api {
    client: Rc<RedmineClient>,
}
impl Api {
    pub fn new(client: Rc<RedmineClient>) -> Api {
        Api {
            client: client,
        }
    }

    pub fn list(&self) -> Result<TimeEntryList> {
        let result = self.client.get("/time_entries.json", &HashMap::new())?;

        serde_json::from_str(&result).chain_err(|| "Can't parse json")
    }

    pub fn create(&self, time_entry: &TimeEntry) -> Result<bool> {
        self.client.create("/time_entries.json", &CreateTimeEntry {
            time_entry: time_entry
        })
    }
}

#[derive(Serialize)]
struct CreateTimeEntry<'a> {
    time_entry: &'a TimeEntry<'a>,
}

#[derive(Default, Serialize)]
pub struct TimeEntry<'a> {
    issue_id: u32,
    hours: f32,
    activity_id: u8,
    comments: &'a str,
    spent_on: Option<&'a str>,
}
impl<'a> TimeEntry<'a> {
    pub fn new(issue_id: u32,
               hours: f32,
               activity_id: u8) -> Self {
        TimeEntry {
            issue_id: issue_id,
            hours: hours,
            activity_id: activity_id,
            ..Default::default()
        }
    }

    pub fn issue_id(mut self, id: u32) -> Self {
        self.issue_id = id;
        self
    }

    pub fn hours(mut self, h: f32) -> Self {
        self.hours = h;
        self
    }

    pub fn activity_id(mut self, id: u8) -> Self {
        self.activity_id = id;
        self
    }

    pub fn comments(mut self, c: &'a str) -> Self {
        self.comments = c;
        self
    }

    pub fn spent_on(mut self, so: &'a str) -> Self {
        self.spent_on = Some(so);
        self
    }
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

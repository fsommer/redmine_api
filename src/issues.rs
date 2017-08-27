extern crate serde_json;

use super::NamedObject;
use super::RedmineClient;

pub struct Api {
    client: RedmineClient,
}
impl Api {
    pub fn new(client: RedmineClient) -> Api {
        Api {
            client: client,
        }
    }

    pub fn list(&self) -> IssueList {
        let result = self.client.list("/issues.json");

        serde_json::from_str(&result).unwrap()
    }
//
//    pub fn create(&self, time_entry: &TimeEntry) -> bool {
//        self.client.create("/time_entries.json", &CreateTimeEntry {
//            time_entry: time_entry
//        })
//    }
}

#[derive(Deserialize, Debug)]
pub struct IssueList {
    issues: Vec<IssueListItem>,
}

#[derive(Deserialize, Debug)]
pub struct IssueListItem {
    pub assigned_to: NamedObject,
    pub author: NamedObject,
    pub created_on: String,
    pub description: String,
    pub done_ratio: u32,
    pub id: u32,
    pub priority: NamedObject,
    pub project: NamedObject,
    pub start_date: String,
    pub status: NamedObject,
    pub subject: String,
    pub tracker: NamedObject,
    pub updated_on: String,
}

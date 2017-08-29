extern crate serde_json;

use std::collections::HashMap;
use std::rc::Rc;
use super::NamedObject;
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

    pub fn list(&self) -> IssueList {
        self.filter().list()
    }

    pub fn filter(&self) -> IssueFilter {
        IssueFilter::new(Rc::clone(&self.client))
    }
//
//    pub fn create(&self, time_entry: &TimeEntry) -> bool {
//        self.client.create("/time_entries.json", &CreateTimeEntry {
//            time_entry: time_entry
//        })
//    }
}

pub struct IssueFilter {
    client: Rc<RedmineClient>,
    assigned_to_id: Option<u32>,
    issue_id: Vec<u32>,
    parent_id: Option<u32>,
    project_id: Option<u32>,
    status_id: Option<u32>,
    subproject_id: Option<u32>,
    tracker_id: Option<u32>,
}
impl IssueFilter {
    fn new(client: Rc<RedmineClient>) -> IssueFilter {
        IssueFilter {
            client: client,
            assigned_to_id: None,
            issue_id: Vec::new(),
            parent_id: None,
            project_id: None,
            status_id: None,
            subproject_id: None,
            tracker_id: None,
        }
    }
//
//    pub fn with_issue_id(&mut self, id: u32) -> &mut IssueFilter {
//        self.issue_id.push(id);
//        self
//    }

    pub fn with_tracker_id(&mut self, id: u32) -> &mut IssueFilter {
        self.tracker_id = Some(id);
        self
    }

    pub fn list(&self) -> IssueList {
        let mut params: HashMap<&str, String> = HashMap::new();
//
//        if self.issue_id.len() > 0 {
//            let issue_id: String = self.issue_id.join(",");
//            params.insert("issue_id", issue_id);
//        }

        if let Some(id) = self.tracker_id {
            params.insert("tracker_id", id.to_string());
        }

        let result = self.client.list("/issues.json", &params);

        serde_json::from_str(&result).unwrap()
    }
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

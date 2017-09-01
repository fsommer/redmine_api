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

    pub fn list(&self) -> Result<IssueList> {
        self.filter().list()
    }

    pub fn filter(&self) -> IssueFilter {
        IssueFilter::new(Rc::clone(&self.client))
    }

    pub fn create(&self, issue: &Issue) -> Result<bool> {
        self.client.create("/issues.json", &CreateIssue {
            issue: issue
        })
    }
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

    pub fn with_assigned_to_id(&mut self, id: u32) -> &mut IssueFilter {
        self.assigned_to_id = Some(id);
        self
    }

    pub fn with_issue_id(&mut self, id: u32) -> &mut IssueFilter {
        self.issue_id.push(id);
        self
    }

    pub fn with_issue_ids(&mut self, ids: Vec<u32>) -> &mut IssueFilter {
        self.issue_id.extend(ids);
        self
    }

    pub fn with_parent_id(&mut self, id: u32) -> &mut IssueFilter {
        self.parent_id = Some(id);
        self
    }

    pub fn with_project_id(&mut self, id: u32) -> &mut IssueFilter {
        self.project_id = Some(id);
        self
    }

    pub fn with_status_id(&mut self, id: u32) -> &mut IssueFilter {
        self.status_id = Some(id);
        self
    }

    pub fn with_subproject_id(&mut self, id: u32) -> &mut IssueFilter {
        self.subproject_id = Some(id);
        self
    }

    pub fn with_tracker_id(&mut self, id: u32) -> &mut IssueFilter {
        self.tracker_id = Some(id);
        self
    }

    pub fn list(&self) -> Result<IssueList> {
        let mut params: HashMap<&str, String> = HashMap::new();

        if let Some(id) = self.assigned_to_id {
            params.insert("assigned_to_id", id.to_string());
        }

        if self.issue_id.len() > 0 {
            let issue_id = self.issue_id.iter().map(|i| i.to_string())
                .collect::<Vec<String>>().join(",");
            params.insert("issue_id", issue_id);
        }

        if let Some(id) = self.parent_id {
            params.insert("parent_id", id.to_string());
        }

        if let Some(id) = self.project_id {
            params.insert("project_id", id.to_string());
        }

        if let Some(id) = self.status_id {
            params.insert("status_id", id.to_string());
        }

        if let Some(id) = self.subproject_id {
            params.insert("subproject_id", id.to_string());
        }

        if let Some(id) = self.tracker_id {
            params.insert("tracker_id", id.to_string());
        }

        let result = self.client.list("/issues.json", &params)?;

        serde_json::from_str(&result).chain_err(|| "Can't parse json")
    }
}

#[derive(Serialize)]
struct CreateIssue<'a> {
    issue: &'a Issue<'a>,
}

#[derive(Serialize)]
pub struct Issue<'a> {
    pub project_id: u32,
    pub tracker_id: u32,
    pub status_id: u32,
    pub priority_id: u32,
    pub subject: &'a str,
    pub description: &'a str,
    pub category_id: u32,
    pub fixed_version_id: u32,
    pub assigned_to_id: u32,
    pub parent_issue_id: u32,
    pub watcher_user_ids: Vec<u32>,
    pub is_private: bool,
    pub estimated_hours: f32,
}

#[derive(Deserialize, Debug)]
pub struct IssueList {
    issues: Vec<IssueListItem>,
}
impl IntoIterator for IssueList {
    type Item = IssueListItem;
    type IntoIter = ::std::vec::IntoIter<IssueListItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.issues.into_iter()
    }
}

#[derive(Deserialize, Debug)]
pub struct IssueListItem {
    pub assigned_to: Option<NamedObject>,
    pub author: NamedObject,
    pub category: Option<NamedObject>,
    pub created_on: String,
    pub description: String,
    pub done_ratio: u32,
    pub due_date: Option<String>,
    pub estimated_hours: Option<f32>,
    pub fixed_version: Option<NamedObject>,
    pub id: u32,
    pub parent: Option<Object>,
    pub priority: NamedObject,
    pub project: NamedObject,
    pub start_date: Option<String>,
    pub status: NamedObject,
    pub subject: String,
    pub tracker: NamedObject,
    pub updated_on: String,
}

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

    pub fn show(&self, id: u32) -> Result<IssueShow> {
        let result = self.client.get(
            &(format!("/issues/{}.json", id)),
            &HashMap::new())?;

        serde_json::from_str(&result).chain_err(|| "Can't parse json")
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

#[derive(Default)]
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
            ..Default::default()
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

        let result = self.client.get("/issues.json", &params)?;

        serde_json::from_str(&result).chain_err(|| "Can't parse json")
    }
}

#[derive(Serialize)]
struct CreateIssue<'a> {
    issue: &'a Issue<'a>,
}

#[derive(Default, Serialize)]
pub struct Issue<'a> {
    project_id: u32,
    tracker_id: u32,
    status_id: u32,
    priority_id: u32,
    subject: &'a str,
    description: &'a str,
    category_id: Option<u32>,
    fixed_version_id: Option<u32>,
    assigned_to_id: Option<u32>,
    parent_issue_id: Option<u32>,
    watcher_user_ids: Vec<u32>,
    is_private: bool,
    estimated_hours: Option<f32>,
}
impl<'a> Issue<'a> {
    pub fn new(project_id: u32,
               tracker_id: u32,
               status_id: u32,
               priority_id: u32,
               subject: &'a str) -> Self {
        Issue {
            project_id: project_id,
            tracker_id: tracker_id,
            status_id: status_id,
            priority_id: priority_id,
            subject: subject,
            ..Default::default()
        }
    }

    pub fn project_id(mut self, id: u32) -> Self {
        self.project_id = id;
        self
    }

    pub fn tracker_id(mut self, id: u32) -> Self {
        self.tracker_id = id;
        self
    }

    pub fn status_id(mut self, id: u32) -> Self {
        self.status_id = id;
        self
    }

    pub fn priority_id(mut self, id: u32) -> Self {
        self.priority_id = id;
        self
    }

    pub fn subject(mut self, s: &'a str) -> Self {
        self.subject = s;
        self
    }

    pub fn description(mut self, s: &'a str) -> Self {
        self.description = s;
        self
    }

    pub fn category_id(mut self, id: u32) -> Self {
        self.category_id = Some(id);
        self
    }

    pub fn fixed_version_id(mut self, id: u32) -> Self {
        self.fixed_version_id = Some(id);
        self
    }

    pub fn assigned_to_id(mut self, id: u32) -> Self {
        self.assigned_to_id = Some(id);
        self
    }

    pub fn parent_issue_id(mut self, id: u32) -> Self {
        self.parent_issue_id = Some(id);
        self
    }

    pub fn watcher_user_ids(mut self, ids: Vec<u32>) -> Self {
        self.watcher_user_ids = ids;
        self
    }

    pub fn add_watcher_user_id(mut self, id: u32) -> Self {
        self.watcher_user_ids.push(id);
        self
    }

    pub fn is_private(mut self, b: bool) -> Self {
        self.is_private = b;
        self
    }

    pub fn estimated_hours(mut self, eh: f32) -> Self {
        self.estimated_hours = Some(eh);
        self
    }
}

#[derive(Deserialize, Debug)]
pub struct IssueList {
    issues: Vec<IssueItem>,
}
impl IntoIterator for IssueList {
    type Item = IssueItem;
    type IntoIter = ::std::vec::IntoIter<IssueItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.issues.into_iter()
    }
}

#[derive(Deserialize, Debug)]
pub struct IssueShow {
    issue: IssueItem,
}

#[derive(Deserialize, Debug)]
pub struct IssueItem {
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

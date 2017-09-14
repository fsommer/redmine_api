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
        Api { client: client }
    }

    pub fn list(&self) -> IssueFilter {
        IssueFilter::new(Rc::clone(&self.client))
    }

    pub fn show(&self, id: u32) -> IssueShow {
        IssueShow {
            client: Rc::clone(&self.client),
            show_id: id,
            ..Default::default()
        }
    }

    pub fn create<'a>(
        &self,
        project_id: u32,
        tracker_id: u32,
        status_id: u32,
        priority_id: u32,
        subject: &'a str,
    ) -> IssueBuilder<'a> {
        IssueBuilder::for_create(
            Rc::clone(&self.client),
            project_id,
            tracker_id,
            status_id,
            priority_id,
            subject,
        )
    }

    pub fn update(&self, id: u32) -> IssueBuilder {
        IssueBuilder::for_update(Rc::clone(&self.client), id)
    }

    pub fn delete(&self, id: u32) -> IssueDelete {
        IssueDelete {
            client: Rc::clone(&self.client),
            delete_id: id,
        }
    }

    pub fn add_watcher(&self, issue_id: u32, watcher_id: u32) -> IssueAddWatcher {
        IssueAddWatcher {
            client: Rc::clone(&self.client),
            issue_id: issue_id,
            watcher_id: watcher_id,
        }
    }

    pub fn remove_watcher(&self, issue_id: u32, watcher_id: u32) -> IssueRemoveWatcher {
        IssueRemoveWatcher {
            client: Rc::clone(&self.client),
            issue_id: issue_id,
            watcher_id: watcher_id,
        }
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

    pub fn assigned_to_id(&mut self, id: u32) -> &mut IssueFilter {
        self.assigned_to_id = Some(id);
        self
    }

    pub fn issue_id(&mut self, id: u32) -> &mut IssueFilter {
        self.issue_id.push(id);
        self
    }

    pub fn issue_ids(&mut self, ids: Vec<u32>) -> &mut IssueFilter {
        self.issue_id.extend(ids);
        self
    }

    pub fn parent_id(&mut self, id: u32) -> &mut IssueFilter {
        self.parent_id = Some(id);
        self
    }

    pub fn project_id(&mut self, id: u32) -> &mut IssueFilter {
        self.project_id = Some(id);
        self
    }

    pub fn status_id(&mut self, id: u32) -> &mut IssueFilter {
        self.status_id = Some(id);
        self
    }

    pub fn subproject_id(&mut self, id: u32) -> &mut IssueFilter {
        self.subproject_id = Some(id);
        self
    }

    pub fn tracker_id(&mut self, id: u32) -> &mut IssueFilter {
        self.tracker_id = Some(id);
        self
    }

    pub fn execute(&self) -> Result<IssueList> {
        let mut params: HashMap<&str, String> = HashMap::new();

        if let Some(id) = self.assigned_to_id {
            params.insert("assigned_to_id", id.to_string());
        }

        if self.issue_id.len() > 0 {
            let issue_id = self.issue_id
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",");
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

#[derive(Deserialize, Debug)]
pub struct IssueList {
    issues: Vec<Issue>,
}
impl IntoIterator for IssueList {
    type Item = Issue;
    type IntoIter = ::std::vec::IntoIter<Issue>;

    fn into_iter(self) -> Self::IntoIter {
        self.issues.into_iter()
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct IssueShow {
    // internal
    #[serde(skip_deserializing)]
    client: Rc<RedmineClient>,
    #[serde(skip_deserializing)]
    show_id: u32,

    // show
    issue: Issue,
}
impl IssueShow {
    pub fn execute(&self) -> Result<Issue> {
        let result = self.client.get(
            &(format!("/issues/{}.json", self.show_id)),
            &HashMap::new(),
        )?;

        Ok(
            serde_json::from_str::<IssueShow>(&result)
                .chain_err(|| "Can't parse json")?
                .into(),
        )
    }
}

pub struct IssueDelete {
    client: Rc<RedmineClient>,
    delete_id: u32,
}
impl IssueDelete {
    pub fn execute(&self) -> Result<bool> {
        self.client.delete(
            &(format!("/issues/{}.json", self.delete_id)),
        )
    }
}

pub struct IssueAddWatcher {
    client: Rc<RedmineClient>,
    issue_id: u32,
    watcher_id: u32,
}
impl IssueAddWatcher {
    pub fn execute(&self) -> Result<bool> {
        #[derive(Serialize)]
        struct Wrapper {
            user_id: u32,
        }

        let response = self.client.post(
            &(format!(
                "/issues/{}/watchers.json",
                self.issue_id
            )),
            &Wrapper { user_id: self.watcher_id },
        )?;

        if !response.status().is_success() {
            bail!("Error: {}", response.status());
        }

        Ok(true)
    }
}

pub struct IssueRemoveWatcher {
    client: Rc<RedmineClient>,
    issue_id: u32,
    watcher_id: u32,
}
impl IssueRemoveWatcher {
    pub fn execute(&self) -> Result<bool> {
        self.client.delete(
            &(format!(
                "/issues/{}/watchers/{}.json",
                self.issue_id,
                self.watcher_id
            )),
        )
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Issue {
    pub assigned_to: Option<NamedObject>,
    pub author: NamedObject,
    pub category: Option<NamedObject>,
    pub created_on: String,
    pub description: Option<String>,
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
impl From<IssueShow> for Issue {
    fn from(item: IssueShow) -> Self {
        item.issue
    }
}

#[derive(Serialize)]
struct IssueBuilderWrapper<'a> {
    issue: &'a IssueBuilder<'a>,
}

#[derive(Debug)]
enum IssueBuilderKind {
    Create,
    Update,
}
impl Default for IssueBuilderKind {
    fn default() -> IssueBuilderKind {
        IssueBuilderKind::Create
    }
}

#[derive(Debug, Default, Serialize)]
pub struct IssueBuilder<'a> {
    // internal
    #[serde(skip_serializing)]
    client: Rc<RedmineClient>,
    #[serde(skip_serializing)]
    kind: IssueBuilderKind,

    // create
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracker_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority_id: Option<u32>,
    #[serde(skip_serializing_if = "str::is_empty")]
    subject: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    description: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    category_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_version_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assigned_to_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_issue_id: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    watcher_user_ids: Vec<u32>,
    is_private: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_hours: Option<f32>,

    // update
    #[serde(skip_serializing)]
    update_id: u32,
    #[serde(skip_serializing_if = "str::is_empty")]
    notes: &'a str,
    private_notes: bool,
}
impl<'a> IssueBuilder<'a> {
    pub fn for_create(
        client: Rc<RedmineClient>,
        project_id: u32,
        tracker_id: u32,
        status_id: u32,
        priority_id: u32,
        subject: &'a str,
    ) -> Self {
        IssueBuilder {
            client: client,
            kind: IssueBuilderKind::Create,

            project_id: Some(project_id),
            tracker_id: Some(tracker_id),
            status_id: Some(status_id),
            priority_id: Some(priority_id),
            subject: subject,
            ..Default::default()
        }
    }

    pub fn for_update(client: Rc<RedmineClient>, id: u32) -> Self {
        IssueBuilder {
            client: client,
            kind: IssueBuilderKind::Update,
            update_id: id,
            ..Default::default()
        }
    }

    pub fn project_id(mut self, id: u32) -> Self {
        self.project_id = Some(id);
        self
    }

    pub fn tracker_id(mut self, id: u32) -> Self {
        self.tracker_id = Some(id);
        self
    }

    pub fn status_id(mut self, id: u32) -> Self {
        self.status_id = Some(id);
        self
    }

    pub fn priority_id(mut self, id: u32) -> Self {
        self.priority_id = Some(id);
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

    pub fn notes(mut self, n: &'a str) -> Self {
        self.notes = n;
        self
    }

    pub fn private_notes(mut self, b: bool) -> Self {
        self.private_notes = b;
        self
    }

    pub fn execute(&self) -> Result<String> {
        let issue = IssueBuilderWrapper { issue: self };
        match self.kind {
            IssueBuilderKind::Create => self.client.create("/issues.json", &issue),
            IssueBuilderKind::Update => {
                self.client.update(
                    &(format!("/issues/{}.json", self.update_id)),
                    &issue,
                )
            }
        }
    }
}

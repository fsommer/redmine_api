//! This module holds everything needed to represent the redmine issues api as described by
//! following link: http://www.redmine.org/projects/redmine/wiki/Rest_Issues.

extern crate serde_json;

use std::collections::HashMap;
use std::rc::Rc;
use super::errors::*;
use super::{Object, NamedObject, RedmineClient};

/// This struct exposes all methods provided by the redmine issues api.
pub struct Api {
    client: Rc<RedmineClient>,
}
impl Api {
    /// Creates a new instance. Should not be called externally.
    pub fn new(client: Rc<RedmineClient>) -> Api {
        Api { client: client }
    }

    /// Returns a filter struct (builder pattern) which ultimately leads to an issue list.
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
    /// let result = redmine.issues().list().status_id(1).execute();
    /// ```
    pub fn list(&self) -> IssueFilter {
        IssueFilter::new(Rc::clone(&self.client))
    }

    /// Returns a single issue by id.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the id of the requested issue
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
    pub fn show(&self, id: u32) -> IssueShow {
        IssueShow {
            client: Rc::clone(&self.client),
            show_id: id,
            ..Default::default()
        }
    }

    /// Returns an IssueBuilder (builder pattern) and ultimately creates a new issue in the redmine
    /// application. The function takes the mandatory information for creating a new issue as
    /// arguments.
    ///
    /// # Arguments
    ///
    /// * `project_id` - an integer holding the project id
    /// * `tracker_id` - an integer holding the tracker id
    /// * `status_id` - an integer holding the status id
    /// * `priority_id` - an integer holding the priority id
    /// * `subject` - a string slice holding the subject
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
    /// let result = redmine.issues().create(1, 1, 1, 1, "my subject")
    ///     .parent_issue_id(3)
    ///     .is_private(true)
    ///     .estimated_hours(3.4)
    ///     .execute();
    ///
    /// ```
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

    /// Returns an IssueBuilder (builder pattern) and ultimately updates an existing issue in the
    /// redmine application. The function takes the id of the issue which should be updated.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the issue id
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
    /// let result = redmine.issues().update(1)
    ///     .notes("This is a new note.")
    ///     .execute();
    ///
    /// ```
    pub fn update(&self, id: u32) -> IssueBuilder {
        IssueBuilder::for_update(Rc::clone(&self.client), id)
    }

    /// Returns IssueDelete struct which offers an `execute` function which deletes the issue
    /// specified by `id` parameter.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the issue id
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
    /// let result = redmine.issues().delete(1).execute();
    /// ```
    pub fn delete(&self, id: u32) -> IssueDelete {
        IssueDelete {
            client: Rc::clone(&self.client),
            delete_id: id,
        }
    }

    /// Returns IssueAddWatcher struct which offers an `execute` function which adds an user as
    /// watcher to an issue.
    ///
    /// # Arguments
    ///
    /// * `issue_id` - an integer holding the issue id
    /// * `watcher_id` - an integer holding the user id
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
    /// let result = redmine.issues().add_watcher(1, 1).execute();
    /// ```
    pub fn add_watcher(&self, issue_id: u32, watcher_id: u32) -> IssueAddWatcher {
        IssueAddWatcher {
            client: Rc::clone(&self.client),
            issue_id: issue_id,
            watcher_id: watcher_id,
        }
    }

    /// Returns IssueRemoveWatcher struct which offers an `execute` function which removes an user
    /// as watcher of an issue.
    ///
    /// # Arguments
    ///
    /// * `issue_id` - an integer holding the issue id
    /// * `watcher_id` - an integer holding the user id
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
    /// let result = redmine.issues().remove_watcher(1, 1).execute();
    /// ```
    pub fn remove_watcher(&self, issue_id: u32, watcher_id: u32) -> IssueRemoveWatcher {
        IssueRemoveWatcher {
            client: Rc::clone(&self.client),
            issue_id: issue_id,
            watcher_id: watcher_id,
        }
    }
}

/// Holds parameters the issues in redmine application should be filtered by and implements a
/// builder patern. Is used as return type for issues.list function.
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
    /// Creates a new instance.
    ///
    /// # Arguments
    ///
    /// * `client` - a Rc boxed RedmineClient
    fn new(client: Rc<RedmineClient>) -> IssueFilter {
        IssueFilter {
            client: client,
            ..Default::default()
        }
    }

    /// Sets filter to get only issues which are assigned to a specific user.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding a user id
    pub fn assigned_to_id(&mut self, id: u32) -> &mut IssueFilter {
        self.assigned_to_id = Some(id);
        self
    }

    /// Sets filter to get only issues specified by id. The function takes a single id and adds it
    /// to a vector of ids which may be holding other issue ids added to the filter previously.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding an issue id
    pub fn issue_id(&mut self, id: u32) -> &mut IssueFilter {
        self.issue_id.push(id);
        self
    }

    /// Sets filter to get only issues specified by ids. The function takes a vector of ids and
    /// pushes it to a vector of ids which may be holding other issue ids added to the filter
    /// previously.
    ///
    /// # Arguments
    ///
    /// * `ids` - a vector holding one or more issue ids
    pub fn issue_ids(&mut self, ids: Vec<u32>) -> &mut IssueFilter {
        self.issue_id.extend(ids);
        self
    }

    /// Sets filter to get only issues which belong to a parent issue specified by `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the id of the parent issue
    pub fn parent_id(&mut self, id: u32) -> &mut IssueFilter {
        self.parent_id = Some(id);
        self
    }

    /// Sets filter to get only issues which belong to a parent issue specified by `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the id of the parent issue
    pub fn project_id(&mut self, id: u32) -> &mut IssueFilter {
        self.project_id = Some(id);
        self
    }

    /// Sets filter to get only issues with a specific status.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the id of the status
    pub fn status_id(&mut self, id: u32) -> &mut IssueFilter {
        self.status_id = Some(id);
        self
    }

    /// Sets filter to get only issues of a specific subproject.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the id of the status
    pub fn subproject_id(&mut self, id: u32) -> &mut IssueFilter {
        self.subproject_id = Some(id);
        self
    }

    /// Sets filter to get only issues with a specific tracker id.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the id of the tracker state
    pub fn tracker_id(&mut self, id: u32) -> &mut IssueFilter {
        self.tracker_id = Some(id);
        self
    }

    /// Performs request to redmine application and returns a list of issues matching the filter
    /// parameters.
    pub fn execute(&self) -> Result<IssueList> {
        let mut params: HashMap<&str, String> = HashMap::new();

        if let Some(id) = self.assigned_to_id {
            params.insert("assigned_to_id", id.to_string());
        }

        if self.issue_id.len() > 0 {
            // transform vector of integers to comma-separated string
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

/// Holds a vector of [Issue](struct.Issue.html)s. Implements IntoIterator trait for easy
/// iteration.
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

/// Wrapper struct for deserialization of a single issue pulled from redmine application.
#[derive(Deserialize, Debug, Default)]
pub struct IssueShow {
    #[serde(skip_deserializing)]
    client: Rc<RedmineClient>,
    #[serde(skip_deserializing)]
    show_id: u32,

    // fields used for deserialization
    issue: Issue,
}
impl IssueShow {
    /// Performs request to redmine application and returns a single issue.
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

/// Helper struct to provide a unified interface for all issue api methods.
pub struct IssueDelete {
    client: Rc<RedmineClient>,
    delete_id: u32,
}
impl IssueDelete {
    /// Performs request to redmine application and deletes an issue.
    pub fn execute(&self) -> Result<bool> {
        self.client.delete(
            &(format!("/issues/{}.json", self.delete_id)),
        )
    }
}

/// Helper struct to provide a unified interface for all issue api methods.
pub struct IssueAddWatcher {
    client: Rc<RedmineClient>,
    issue_id: u32,
    watcher_id: u32,
}
impl IssueAddWatcher {
    /// Performs request to redmine application and adds a user as watcher to an issue.
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

/// Helper struct to provide a unified interface for all issue api methods.
pub struct IssueRemoveWatcher {
    client: Rc<RedmineClient>,
    issue_id: u32,
    watcher_id: u32,
}
impl IssueRemoveWatcher {
    /// Performs request to redmine application and removes a user as watcher from an issue.
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

/// Represents an issue as pulled from redmine application.
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

/// Helper struct for serialization.
#[derive(Serialize)]
struct IssueBuilderWrapper<'a> {
    issue: &'a IssueBuilder<'a>,
}

/// Enumeration for differentiation between creation and update.
#[derive(Debug)]
enum IssueBuilderKind {
    Create,
    Update,
}
// IssueBuilder implements Default trait, so IssueBuilderKind has to implement Default, too.
impl Default for IssueBuilderKind {
    fn default() -> IssueBuilderKind {
        IssueBuilderKind::Create
    }
}

/// Struct to provide builder pattern for creation and update of issues. Can be serialized to be
/// used as json parameter for request to redmine application.
#[derive(Debug, Default, Serialize)]
pub struct IssueBuilder<'a> {
    // internal
    #[serde(skip_serializing)]
    client: Rc<RedmineClient>,
    #[serde(skip_serializing)]
    kind: IssueBuilderKind,

    // fields used for serialization needed for creation
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

    // additional fields used for serialization needed for update
    #[serde(skip_serializing)]
    update_id: u32,
    #[serde(skip_serializing_if = "str::is_empty")]
    notes: &'a str,
    private_notes: bool,
}
impl<'a> IssueBuilder<'a> {
    /// Creates new instance for creation of an issue. Function takes all mandatory parameters for
    /// a new issue.
    ///
    /// # Arguments
    ///
    /// * `client` - an Rc boxed [RedmineClient](struct.RedmineClient.html)
    /// * `project_id` - an integer holding the project id
    /// * `tracker_id` - an integer holding the tracker id
    /// * `status_id` - an integer holding the status id
    /// * `priority_id` - an integer holding the priority id
    /// * `subject` - a string slice holding the subject
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

    /// Creates new instance for update of an issue. Function takes mandatory parameter for update:
    /// an id of an issue.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the issue id
    pub fn for_update(client: Rc<RedmineClient>, id: u32) -> Self {
        IssueBuilder {
            client: client,
            kind: IssueBuilderKind::Update,
            update_id: id,
            ..Default::default()
        }
    }

    /// Sets project id for issue.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the issue id
    pub fn project_id(mut self, id: u32) -> Self {
        self.project_id = Some(id);
        self
    }

    /// Sets tracker id for issue.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the tracker id
    pub fn tracker_id(mut self, id: u32) -> Self {
        self.tracker_id = Some(id);
        self
    }

    /// Sets status id for issue.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the status id
    pub fn status_id(mut self, id: u32) -> Self {
        self.status_id = Some(id);
        self
    }

    /// Sets priority id for issue.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the priority id
    pub fn priority_id(mut self, id: u32) -> Self {
        self.priority_id = Some(id);
        self
    }

    /// Sets subject for issue.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the subject
    pub fn subject(mut self, s: &'a str) -> Self {
        self.subject = s;
        self
    }

    /// Sets description for issue.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the description
    pub fn description(mut self, s: &'a str) -> Self {
        self.description = s;
        self
    }

    /// Sets category id for issue.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the category id
    pub fn category_id(mut self, id: u32) -> Self {
        self.category_id = Some(id);
        self
    }

    /// Sets version id for issue.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the version id
    pub fn fixed_version_id(mut self, id: u32) -> Self {
        self.fixed_version_id = Some(id);
        self
    }

    /// Sets assignee for issue.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the user id
    pub fn assigned_to_id(mut self, id: u32) -> Self {
        self.assigned_to_id = Some(id);
        self
    }

    /// Sets parent issue for issue.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the issue id of the parent
    pub fn parent_issue_id(mut self, id: u32) -> Self {
        self.parent_issue_id = Some(id);
        self
    }

    /// Sets multiple users as watchers for issue.
    ///
    /// # Arguments
    ///
    /// * `ids` - a vector of user ids
    pub fn watcher_user_ids(mut self, ids: Vec<u32>) -> Self {
        self.watcher_user_ids = ids;
        self
    }

    /// Adds a single user as watcher to the issue.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the user id
    pub fn add_watcher_user_id(mut self, id: u32) -> Self {
        self.watcher_user_ids.push(id);
        self
    }

    /// Sets privacy status for issue.
    ///
    /// # Arguments
    ///
    /// * `b` - a boolean: true means private, false means public
    pub fn is_private(mut self, b: bool) -> Self {
        self.is_private = b;
        self
    }

    /// Sets estimated hours of the issue.
    ///
    /// # Arguments
    ///
    /// * `eh` - a floating point number holding the estimated hours
    pub fn estimated_hours(mut self, eh: f32) -> Self {
        self.estimated_hours = Some(eh);
        self
    }

    /// Adds note to the issue.
    ///
    /// # Arguments
    ///
    /// * `n` - a string slice holding the note
    pub fn notes(mut self, n: &'a str) -> Self {
        self.notes = n;
        self
    }

    /// Adds privacy status to the issue note.
    ///
    /// # Arguments
    ///
    /// * `b` - a boolean: true means it's a private note, false means it's a public note
    pub fn private_notes(mut self, b: bool) -> Self {
        self.private_notes = b;
        self
    }

    /// Performs request to redmine application to create or update an issue.
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

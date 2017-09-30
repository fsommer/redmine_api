//! Holds some functions to represent the redmine time entries api partially as described by
//! the following link: http://www.redmine.org/projects/redmine/wiki/Rest_TimeEntries

extern crate serde_json;

use std::collections::HashMap;
use std::rc::Rc;
use super::errors::*;
use super::{Object, NamedObject, RedmineClient};

/// Exposes all methods provided by the redmine time entries api as implemented so far.
pub struct Api {
    client: Rc<RedmineClient>,
}
impl Api {
    /// Creates a new instance. Should not be called externally.
    pub fn new(client: Rc<RedmineClient>) -> Api {
        Api { client: client }
    }

    /// Returns a list of time entries.
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
    /// let result = redmine.time_entries().list().user_id(1).execute();
    /// ```
    pub fn list(&self) -> TimeEntryFilter {
        TimeEntryFilter::new(Rc::clone(&self.client))
    }

    /// Creates a new time entry in the redmine application.
    ///
    /// # Arguments
    ///
    /// * `issue_id` - an integer holding the issue id
    /// * `hours` - an floating point number holding the spent hours
    /// * `activity_id` - an integer holding the activity id
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
    /// let result = redmine.time_entries().create(1, 0.2, 4)
    ///     .comments("Hello World")
    ///     .spent_on("2017-09-16")
    ///     .execute();
    /// ```
    pub fn create(
        &self,
        issue_id: u32,
        hours: f32,
        activity_id: u32,
    ) -> TimeEntryBuilder {
        TimeEntryBuilder::for_create(
            Rc::clone(&self.client),
            issue_id,
            hours,
            activity_id,
        )
    }
}

/// Holds parameters the time entries in redmine application should be filtered by and implements
/// builder pattern. Is used as return type by time_entries.list function.
#[derive(Default)]
pub struct TimeEntryFilter {
    client: Rc<RedmineClient>,
    user_id: Option<u32>,
    project_id: Option<u32>,
}
impl TimeEntryFilter {
    /// Creates new instance.
    ///
    /// # Arguments
    ///
    /// * `client` - a Rc boxed RedmineClient
    fn new(client: Rc<RedmineClient>) -> Self {
        TimeEntryFilter {
            client: client,
            ..Default::default()
        }
    }

    /// Sets filter to get only time entries which belong to a specific user.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding a user id
    pub fn user_id(&mut self, id: u32) -> &mut Self {
        self.user_id = Some(id);
        self
    }

    /// Sets filter to get only time entries which belong to a specific project.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding a project id
    pub fn project_id(&mut self, id: u32) -> &mut Self {
        self.project_id = Some(id);
        self
    }

    /// Performs request to redmine application and returns a list of time entries matching the
    /// filter parameters.
    pub fn execute(&self) -> Result<TimeEntryList> {
        let mut params: HashMap<&str, String> = HashMap::new();

        if let Some(id) = self.user_id {
            params.insert("user_id", id.to_string());
        }

        if let Some(id) = self.project_id {
            params.insert("project_id", id.to_string());
        }

        let result = self.client.get("/time_entries.json", &params)?;

        serde_json::from_str(&result).chain_err(|| "Can't parse json")
    }
}

/// Holds a vector of [TimeEntryListItem](struct.TimeEntryList.html)s.
#[derive(Deserialize, Debug)]
pub struct TimeEntryList {
    time_entries: Vec<TimeEntryListItem>,
}

/// Represents a time entry as fetched from redmine application.
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


/// Helper struct for serialization.
#[derive(Serialize)]
struct TimeEntryBuilderWrapper<'a> {
    time_entry: &'a TimeEntryBuilder<'a>,
}

/// Struct to provide builder pattern for creation of time entries. Can be serialized to be used as
/// json parameter for request to redmine application.
#[derive(Debug, Default, Serialize)]
pub struct TimeEntryBuilder<'a> {
    // internal
    #[serde(skip_serializing)]
    client: Rc<RedmineClient>,

    // fields used for serialization needed for creation
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_id: Option<u32>,
    #[serde(skip_serializing_if = "str::is_empty")]
    spent_on: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    comments: &'a str,
}
impl<'a> TimeEntryBuilder<'a> {
    /// Creates new instance for creation of a time entry. Function takes all mandatory parameters
    /// for a new time entry.
    ///
    /// # Arguments
    ///
    /// * `client` - an Rc boxed [RedmineClient](struct.RedmineClient.html)
    /// * `issue_id` - an integer holding the issue id
    /// * `hours` - an floating point number holding the spent hours
    /// * `activity_id` - an integer holding the activity id
    pub fn for_create(
        client: Rc<RedmineClient>,
        issue_id: u32,
        hours: f32,
        activity_id: u32,
    ) -> Self {
        TimeEntryBuilder {
            client: client,

            issue_id: Some(issue_id),
            hours: Some(hours),
            activity_id: Some(activity_id),
            ..Default::default()
        }
    }

    /// Sets spent on date for time entry.
    ///
    /// # Arguments
    ///
    /// * `s` - string slice holding the spent on date
    pub fn spent_on(mut self, s: &'a str) -> Self {
        self.spent_on = s;
        self
    }

    /// Sets comment for time entry.
    ///
    /// # Arguments
    ///
    /// * `s` - string slice holding the comment
    pub fn comments(mut self, s: &'a str) -> Self {
        self.comments = s;
        self
    }

    /// Performs request to redmine application to create a time entry.
    pub fn execute(&self) -> Result<String> {
        let te = TimeEntryBuilderWrapper { time_entry: self };
        self.client.create("/time_entries.json", &te)
    }
}

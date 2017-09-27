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

    }

    /// Creates a new time entry in the redmine application.
    ///
    /// # Arguments
    ///
    /// * `time_entry` - a TimeEntry holding all information needed to create a time entry
    ///
    /// # Example
    ///
    /// ```
    /// use redmine_api::RedmineApi;
    /// use redmine_api::time_entries::TimeEntry;
    ///
    /// let redmine = RedmineApi::new(
    ///     "http://www.redmine.org/".to_string(),
    ///     "1234".to_string()
    /// );
    ///
    /// let time_entry = TimeEntry::new(1, 0.2, 4)
    ///     .comments("Hello World")
    ///     .spent_on("2017-09-16");
    ///
    /// let result = redmine.time_entries().create(&time_entry);
    /// ```
    pub fn create(&self, time_entry: &TimeEntry) -> Result<String> {
        self.client.create(
            "/time_entries.json",
            &CreateTimeEntry { time_entry: time_entry },
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

/// Wraps a TimeEntry for serialization.
#[derive(Serialize)]
struct CreateTimeEntry<'a> {
    time_entry: &'a TimeEntry<'a>,
}

/// Represents a time entry.
#[derive(Default, Serialize)]
pub struct TimeEntry<'a> {
    issue_id: u32,
    hours: f32,
    activity_id: u8,
    comments: &'a str,
    spent_on: Option<&'a str>,
}
impl<'a> TimeEntry<'a> {
    /// Creates a new TimeEntry.
    pub fn new(issue_id: u32, hours: f32, activity_id: u8) -> Self {
        TimeEntry {
            issue_id: issue_id,
            hours: hours,
            activity_id: activity_id,
            ..Default::default()
        }
    }

    /// Sets issue id the time entry belongs to.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding issue id
    pub fn issue_id(mut self, id: u32) -> Self {
        self.issue_id = id;
        self
    }

    /// Sets amount of hours for the time entry.
    ///
    /// # Arguments
    ///
    /// `h` - a floating point number holding the amount of hours
    pub fn hours(mut self, h: f32) -> Self {
        self.hours = h;
        self
    }

    /// Sets activity id of the time entry.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the id of the activity
    pub fn activity_id(mut self, id: u8) -> Self {
        self.activity_id = id;
        self
    }

    /// Sets comment for the time entry.
    ///
    /// # Arguments
    ///
    /// * `c` - a string slice holding the comment
    pub fn comments(mut self, c: &'a str) -> Self {
        self.comments = c;
        self
    }

    /// Sets date the time was spent on.
    ///
    /// # Arguments
    ///
    /// * `so` - a string slice holding the date, e.g. "2017-09-16"
    pub fn spent_on(mut self, so: &'a str) -> Self {
        self.spent_on = Some(so);
        self
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

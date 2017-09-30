//! This module holds everything needed to represent the redmine projects api as described by
//! following link: http://www.redmine.org/projects/redmine/wiki/Rest_Projects.

extern crate serde_json;

use std::collections::HashMap;
use std::rc::Rc;
use super::errors::*;
use super::RedmineClient;

/// This struct exposes all methods provided by the redmine projects api.
pub struct Api {
    client: Rc<RedmineClient>,
}
impl Api {
    /// Creates a new instance. Should not be called externally.
    pub fn new(client: Rc<RedmineClient>) -> Api {
        Api { client: client }
    }

    /// Returns ProjectListExecutor struct which provides an `execute` function for retreiving a
    /// list of projects.
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
    /// let result = redmine.projects().list().execute();
    /// ```
    pub fn list(&self) -> ProjectListExecutor {
        ProjectListExecutor::new(Rc::clone(&self.client))
    }

    /// Returns a single project by id.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the id of the requested project
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
    /// let result = redmine.projects().show(1).execute();
    /// ```
    pub fn show(&self, id: u32) -> ProjectShow {
        ProjectShow {
            client: Rc::clone(&self.client),
            show_id: id,
            ..Default::default()
        }
    }

    /// Returns an ProjectBuilder and ultimately creates a new project in the redmine application.
    /// The function takes the mandatory information for creating a new project as arguments.
    ///
    /// # Arguments
    ///
    /// * `name` - a string slice holding the name of the project
    /// * `identifier` - a string slice holding the unique identifier of the project
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
    /// let result = redmine.projects().create("My Project", "my_project")
    ///     .description("An awesome project.")
    ///     .execute();
    /// ```
    pub fn create<'a>(&self, name: &'a str, identifier: &'a str) -> ProjectBuilder<'a> {
        ProjectBuilder::for_create(Rc::clone(&self.client), name, identifier)
    }

    /// Returns an ProjectBuilder and ultimately updates an existing prpoject in the redmine
    /// application. The function takes the id of the project which should be updated.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the project id
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
    /// let result = redmine.projects().update(1)
    ///     .description("This description is not helpful.")
    ///     .execute();
    /// ```
    pub fn update(&self, id: u32) -> ProjectBuilder {
        ProjectBuilder::for_update(Rc::clone(&self.client), id)
    }

    /// Returns ProjectDelete struct which offers an `execute` function which deletes the project
    /// specified by `id` parameter.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the project id
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
    /// let result = redmine.projects().delete(1).execute();
    /// ```
    pub fn delete(&self, id: u32) -> ProjectDelete {
        ProjectDelete {
            client: Rc::clone(&self.client),
            delete_id: id,
        }
    }
}

/// Helper struct to provide a unified interface for all project api methods.
#[derive(Default)]
pub struct ProjectListExecutor {
    client: Rc<RedmineClient>,
}
impl ProjectListExecutor {
    /// Creates a new instance.
    ///
    /// # Arguments
    ///
    /// * `client` - a Rc boxed RedmineClient
    fn new(client: Rc<RedmineClient>) -> Self {
        Self {
            client: client,
        }
    }

    /// Performs request to redmine application and returns a list of projects (accessible by the
    /// user)
    pub fn execute(&self) -> Result<ProjectList> {
        let result = self.client.get("/projects.json", &HashMap::new())?;

        serde_json::from_str(&result).chain_err(|| "Can't parse json")
    }
}

/// Holds a vector of [Project](struct.Project.html)s. Implements IntoIterator trait for easy
/// iteration.
#[derive(Deserialize, Debug)]
pub struct ProjectList {
    projects: Vec<Project>,
}
impl IntoIterator for ProjectList {
    type Item = Project;
    type IntoIter = ::std::vec::IntoIter<Project>;

    fn into_iter(self) -> Self::IntoIter {
        self.projects.into_iter()
    }
}

/// Wrapper struct for deserialization of a single Project pulled from redmine application.
#[derive(Deserialize, Debug, Default)]
pub struct ProjectShow {
    #[serde(skip_deserializing)]
    client: Rc<RedmineClient>,
    #[serde(skip_deserializing)]
    show_id: u32,

    // fields used for deserialization
    project: Project,
}
impl ProjectShow {
    /// Performs request to redmine application and returns a single project.
    pub fn execute(&self) -> Result<Project> {
        let result = self.client.get(
            &(format!("/projects/{}.json", self.show_id)),
            &HashMap::new(),
        )?;

        Ok(
            serde_json::from_str::<ProjectShow>(&result)
                .chain_err(|| "Can't parse json")?
                .into(),
        )
    }
}

/// Helper struct to provide a unified interface for all project api methods.
pub struct ProjectDelete {
    client: Rc<RedmineClient>,
    delete_id: u32,
}
impl ProjectDelete {
    /// Performs request to redmine application and deletes a project.
    pub fn execute(&self) -> Result<bool> {
        self.client.delete(
            &(format!("/projects/{}.json", self.delete_id)),
        )
    }
}

/// Represents a project as pulled from redmine application.
#[derive(Deserialize, Debug, Default)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub identifier: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub status: u32,
    pub is_public: Option<bool>,
    pub created_on: String,
    pub updated_on: String,
}
impl From<ProjectShow> for Project {
    fn from(item: ProjectShow) -> Self {
        item.project
    }
}

/// Helper struct for serialization.
#[derive(Serialize)]
struct ProjectBuilderWrapper<'a> {
    project: &'a ProjectBuilder<'a>,
}

/// Enumeration for differentiation between creation and update.
#[derive(Debug)]
enum ProjectBuilderKind {
    Create,
    Update,
}
// ProjectBuilder implements Default trait, so ProjectBuilderKind has to implement Default, too.
impl Default for ProjectBuilderKind {
    fn default() -> ProjectBuilderKind {
        ProjectBuilderKind::Create
    }
}

/// Struct to provide builder pattern for creation and update of projects. Can be serialized to be
/// used as json parameter for request to redmine application.
#[derive(Debug, Default, Serialize)]
pub struct ProjectBuilder<'a> {
    // internal
    #[serde(skip_serializing)]
    client: Rc<RedmineClient>,
    #[serde(skip_serializing)]
    kind: ProjectBuilderKind,
    #[serde(skip_serializing)]
    update_id: u32,

    // fields used for serialization
    #[serde(skip_serializing_if = "str::is_empty")]
    name: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    identifier: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    description: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    homepage: &'a str,
    is_public: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<u32>,
    inherit_members: bool,
}
impl<'a> ProjectBuilder<'a> {
    /// Creates new instance for creation of a project. Function takes all mandatory parameters for
    /// a new project.
    ///
    /// # Arguments
    ///
    /// * `name` - a string slice holding the name of the project
    /// * `identifier` - a string slice holding the unique identifier of the project
    pub fn for_create(
        client: Rc<RedmineClient>,
        name: &'a str,
        identifier: &'a str,
    ) -> Self {
        ProjectBuilder {
            client: client,
            kind: ProjectBuilderKind::Create,

            name: name,
            identifier: identifier,
            ..Default::default()
        }
    }

    /// Creates new instance for update of a project. Function takes id of the project which should
    /// be updated.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the project id
    pub fn for_update(client: Rc<RedmineClient>, id: u32) -> Self {
        ProjectBuilder {
            client: client,
            kind: ProjectBuilderKind::Update,
            update_id: id,
            ..Default::default()
        }
    }

    /// Sets name for project.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the subject
    pub fn name(mut self, s: &'a str) -> Self {
        self.name = s;
        self
    }

    /// Sets identifier for project.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the subject
    pub fn identifier(mut self, s: &'a str) -> Self {
        self.identifier = s;
        self
    }

    /// Sets description for project.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the subject
    pub fn description(mut self, s: &'a str) -> Self {
        self.description = s;
        self
    }

    /// Sets homepage for project.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the subject
    pub fn homepage(mut self, s: &'a str) -> Self {
        self.homepage = s;
        self
    }

    /// Sets privacy status for project.
    ///
    /// # Arguments
    ///
    /// * `b` - a boolean: true means public, false means private
    pub fn is_public(mut self, b: bool) -> Self {
        self.is_public = b;
        self
    }

    /// Sets parent project for project.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the id of the parent project
    pub fn parent_id(mut self, id: u32) -> Self {
        self.parent_id = Some(id);
        self
    }

    /// Sets if project members should be inherited from parent project.
    ///
    /// # Arguments
    ///
    /// * `b` - a boolean: true if members should be inherited, false otherwise
    pub fn inherit_members(mut self, b: bool) -> Self {
        self.inherit_members = b;
        self
    }

    /// Performs request to redmine application to create or update a project.
    pub fn execute(&self) -> Result<String> {
        let project = ProjectBuilderWrapper { project: self };
        match self.kind {
            ProjectBuilderKind::Create => self.client.create("/projects.json", &project),
            ProjectBuilderKind::Update => {
                self.client.update(
                    &(format!("/projects/{}.json", self.update_id)),
                    &project,
                )
            }
        }
    }
}

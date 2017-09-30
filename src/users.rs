//! This module holds everything needed to represent the redmine users api as described by
//! following link: http://www.redmine.org/projects/redmine/wiki/Rest_Users.

extern crate serde_json;

use std::collections::HashMap;
use std::rc::Rc;
use super::errors::*;
use super::RedmineClient;

/// This struct exposes all methods provided by the redmine users api.
pub struct Api {
    client: Rc<RedmineClient>,
}
impl Api {
    /// Creates a new instance. Should not be called externally.
    pub fn new(client: Rc<RedmineClient>) -> Api {
        Api { client: client }
    }

    /// Returns a filter struct (builder pattern) which ultimately leads to a list of users.
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
    /// let result = redmine.users().list().execute();
    /// ```
    pub fn list(&self) -> UserFilter {
        UserFilter::new(Rc::clone(&self.client))
    }

    /// Returns a single user by id.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the id of the requested user
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
    /// let result = redmine.users().show(1).execute();
    /// ```
    pub fn show(&self, id: u32) -> UserShow {
        UserShow {
            client: Rc::clone(&self.client),
            show_id: id,
            ..Default::default()
        }
    }

    /// Returns an UserBuilder and ultimately creates a new user in the redmine application. The
    /// function takes the mandatory information for creating a new user as arguments.
    ///
    /// # Arguments
    ///
    /// * `login` - a string slice holding the login of the user
    /// * `firstname` - a string slice holding the firstname of the user
    /// * `lastname` - a string slice holding the lastname of the user
    /// * `mail` - a string slice holding the email address of the user
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
    /// let result = redmine.users().create("juser", "jane", "user", "juser@mail.com")
    ///     .password("secret")
    ///     .execute();
    /// ```
    pub fn create<'a>(
        &self,
        login: &'a str,
        firstname: &'a str,
        lastname: &'a str,
        mail: &'a str,
    ) -> UserBuilder<'a> {
        UserBuilder::for_create(Rc::clone(&self.client), login, firstname, lastname, mail)
    }

    /// Returns an UserBuilder and ultimately updates an existing prpoject in the redmine
    /// application. The function takes the id of the user which should be updated.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the user id
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
    /// let result = redmine.users().update(1)
    ///     .firstname("Jane")
    ///     .execute();
    /// ```
    pub fn update(&self, id: u32) -> UserBuilder {
        UserBuilder::for_update(Rc::clone(&self.client), id)
    }

    /// Returns UserDelete struct which offers an `execute` function which deletes the user
    /// specified by `id` parameter.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the user id
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
    /// let result = redmine.users().delete(1).execute();
    /// ```
    pub fn delete(&self, id: u32) -> UserDelete {
        UserDelete {
            client: Rc::clone(&self.client),
            delete_id: id,
        }
    }
}

/// Holds parameters the users in redmine application should be filtered by and implements a
/// builder patern. Is used as return type for users.list function.
/// TODO
#[derive(Default)]
pub struct UserFilter {
    client: Rc<RedmineClient>,
}
impl UserFilter {
    /// Creates a new instance.
    ///
    /// # Arguments
    ///
    /// * `client` - a Rc boxed RedmineClient
    fn new(client: Rc<RedmineClient>) -> Self {
        Self { client: client }
    }

    /// Performs request to redmine application and returns a list of users.
    pub fn execute(&self) -> Result<UserList> {
        let result = self.client.get("/users.json", &HashMap::new())?;

        serde_json::from_str(&result).chain_err(|| "Can't parse json")
    }
}

/// Holds a vector of [User](struct.User.html)s. Implements IntoIterator trait for easy iteration.
#[derive(Deserialize, Debug)]
pub struct UserList {
    users: Vec<User>,
}
impl IntoIterator for UserList {
    type Item = User;
    type IntoIter = ::std::vec::IntoIter<User>;

    fn into_iter(self) -> Self::IntoIter {
        self.users.into_iter()
    }
}

/// Wrapper struct for deserialization of a single User pulled from redmine application.
#[derive(Deserialize, Debug, Default)]
pub struct UserShow {
    #[serde(skip_deserializing)]
    client: Rc<RedmineClient>,
    #[serde(skip_deserializing)]
    show_id: u32,

    // fields used for deserialization
    user: User,
}
impl UserShow {
    /// Performs request to redmine application and returns a single user.
    pub fn execute(&self) -> Result<User> {
        let result = self.client.get(
            &(format!("/users/{}.json", self.show_id)),
            &HashMap::new(),
        )?;

        Ok(
            serde_json::from_str::<UserShow>(&result)
                .chain_err(|| "Can't parse json")?
                .into(),
        )
    }
}

/// Helper struct to provide a unified interface for all user api methods.
pub struct UserDelete {
    client: Rc<RedmineClient>,
    delete_id: u32,
}
impl UserDelete {
    /// Performs request to redmine application and deletes a user.
    pub fn execute(&self) -> Result<bool> {
        self.client.delete(
            &(format!("/users/{}.json", self.delete_id)),
        )
    }
}

/// Represents a user as pulled from redmine application.
#[derive(Deserialize, Debug, Default)]
pub struct User {
    pub id: u32,
    pub login: String,
    pub firstname: String,
    pub lastname: String,
    pub mail: String,
    pub created_on: String,
    pub last_login_on: Option<String>,
}
impl From<UserShow> for User {
    fn from(item: UserShow) -> Self {
        item.user
    }
}

/// Helper struct for serialization.
#[derive(Serialize)]
struct UserBuilderWrapper<'a> {
    user: &'a UserBuilder<'a>,
}

/// Enumeration for differentiation between creation and update.
#[derive(Debug)]
enum UserBuilderKind {
    Create,
    Update,
}
// UserBuilder implements Default trait, so UserBuilderKind has to implement Default, too.
impl Default for UserBuilderKind {
    fn default() -> UserBuilderKind {
        UserBuilderKind::Create
    }
}

/// Struct to provide builder pattern for creation and update of users. Can be serialized to be
/// used as json parameter for request to redmine application.
#[derive(Debug, Default, Serialize)]
pub struct UserBuilder<'a> {
    // internal
    #[serde(skip_serializing)]
    client: Rc<RedmineClient>,
    #[serde(skip_serializing)]
    kind: UserBuilderKind,
    #[serde(skip_serializing)]
    update_id: u32,

    // fields used for serialization
    #[serde(skip_serializing_if = "str::is_empty")]
    login: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    firstname: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    lastname: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    mail: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    password: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_source_id: Option<u32>,
    must_change_passwd: bool,
    generate_password: bool,
}
impl<'a> UserBuilder<'a> {
    /// Creates new instance for creation of a user. Function takes all mandatory parameters for a
    /// new user.
    ///
    /// # Arguments
    ///
    /// * `login` - a string slice holding the login of the user
    /// * `firstname` - a string slice holding the firstname of the user
    /// * `lastname` - a string slice holding the lastname of the user
    /// * `mail` - a string slice holding the email address of the user
    pub fn for_create(
        client: Rc<RedmineClient>,
        login: &'a str,
        firstname: &'a str,
        lastname: &'a str,
        mail: &'a str,
    ) -> Self {
        UserBuilder {
            client: client,
            kind: UserBuilderKind::Create,

            login: login,
            firstname: firstname,
            lastname: lastname,
            mail: mail,
            ..Default::default()
        }
    }

    /// Creates new instance for update of a user. Function takes id of the user which should
    /// be updated.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the user id
    pub fn for_update(client: Rc<RedmineClient>, id: u32) -> Self {
        UserBuilder {
            client: client,
            kind: UserBuilderKind::Update,
            update_id: id,
            ..Default::default()
        }
    }

    /// Sets login for user.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the login
    pub fn login(mut self, s: &'a str) -> Self {
        self.login = s;
        self
    }

    /// Sets firstname for user.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the firstname
    pub fn firstname(mut self, s: &'a str) -> Self {
        self.firstname = s;
        self
    }

    /// Sets lastname for user.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the lastname
    pub fn lastname(mut self, s: &'a str) -> Self {
        self.lastname = s;
        self
    }

    /// Sets mail for user.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the email address
    pub fn mail(mut self, s: &'a str) -> Self {
        self.mail = s;
        self
    }

    /// Sets password for user.
    ///
    /// # Arguments
    ///
    /// * `s` - a string slice holding the password
    pub fn password(mut self, s: &'a str) -> Self {
        self.password = s;
        self
    }

    /// Sets auth_source_id for user.
    ///
    /// # Arguments
    ///
    /// * `id` - an integer holding the auth_source_id
    pub fn auth_source_id(mut self, id: u32) -> Self {
        self.auth_source_id = Some(id);
        self
    }

    /// Sets must_change_passwd for user.
    ///
    /// # Arguments
    ///
    /// * `b` - a boolean: true means user must change passwd after first login
    pub fn must_change_passwd(mut self, b: bool) -> Self {
        self.must_change_passwd = b;
        self
    }

    /// Sets generate_password for user.
    ///
    /// # Arguments
    ///
    /// * `b` - a boolean: true means the password should be automatically generated
    pub fn generate_password(mut self, b: bool) -> Self {
        self.generate_password = b;
        self
    }

    /// Performs request to redmine application to create or update a user.
    pub fn execute(&self) -> Result<String> {
        let user = UserBuilderWrapper { user: self };
        match self.kind {
            UserBuilderKind::Create => self.client.create("/users.json", &user),
            UserBuilderKind::Update => {
                self.client.update(
                    &(format!("/users/{}.json", self.update_id)),
                    &user,
                )
            }
        }
    }
}

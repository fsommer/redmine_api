extern crate redmine_api;

use redmine_api::RedmineApi;

fn main() {
    let redmine = RedmineApi::new(
        "http://localhost:8080".to_string(),
        "96b3ddaa1d27af3f7cb8adf0910e4c954f437917".to_string(),
    );

    let result = redmine
        .users()
        .create("testuser", "test", "user", "test.user@non-existing.com")
        .password("secretpassword")
        .execute();

    println!("Result: {:?}", result);
}

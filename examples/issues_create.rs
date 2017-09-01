extern crate redmine_api;

use redmine_api::RedmineApi;
use redmine_api::issues::Issue;

fn main() {
    let redmine = RedmineApi::new(
        "http://localhost:8080".to_string(),
        "bbde69d1999dde8f497199f49bb7b577389b6c0e".to_string(),
    );

    let issue = Issue::new(1, 1, 1, 1, "subject 2")
        .parent_issue_id(3)
        .is_private(true)
        .estimated_hours(1.1);

    let result = redmine.issues().create(&issue);
    println!("Result: {:?}", result);
}

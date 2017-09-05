extern crate redmine_api;

use redmine_api::RedmineApi;

fn main() {
    let redmine = RedmineApi::new(
        "http://localhost:8080".to_string(),
        "bbde69d1999dde8f497199f49bb7b577389b6c0e".to_string(),
    );

    let result = redmine.issues().create(1, 1, 1, 1, "2017-09-05 subject")
        .parent_issue_id(3)
        .is_private(true)
        .estimated_hours(3.4)
        .execute();

    println!("Result: {:?}", result);
}

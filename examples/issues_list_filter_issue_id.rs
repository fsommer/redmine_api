extern crate redmine_api;

use redmine_api::RedmineApi;

fn main() {
    let redmine = RedmineApi::new(
        "http://localhost:8080".to_string(),
        "bbde69d1999dde8f497199f49bb7b577389b6c0e".to_string(),
    );

    let result = redmine.issues().filter()
        .with_issue_id(1)
        .with_issue_ids(vec![2, 3])
        .list().unwrap();

    for issue in result {
        println!("ID: {}, Subject: {}", issue.id, issue.subject);
    }
}

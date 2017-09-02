extern crate redmine_api;

use redmine_api::RedmineApi;
use redmine_api::issues::Issue;

fn main() {
    let redmine = RedmineApi::new(
        "http://localhost:8080".to_string(),
        "bbde69d1999dde8f497199f49bb7b577389b6c0e".to_string(),
    );

    let issue = Issue::new(1, 1, 1, 1, "changed subject")
        .notes("This is a private note")
        .private_notes(true);

    let result = redmine.issues().update(17, &issue);
    println!("Result: {:?}", result);
}

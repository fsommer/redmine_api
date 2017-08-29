extern crate redmine_api;

use redmine_api::RedmineApi;
use redmine_api::time_entries::TimeEntry;

fn main() {
    let redmine = RedmineApi::new(
        "http://localhost:8080".to_string(),
        "bbde69d1999dde8f497199f49bb7b577389b6c0e".to_string(),
    );

    let time_entry = TimeEntry {
        issue_id: 1,
        hours: 0.1,
        activity_id: 4,
        comments: "Test am Sonntag".to_string(),
    };

    let result = redmine.time_entries().create(&time_entry);
    println!("Result: {}", result);
}

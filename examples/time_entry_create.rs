extern crate redmine_api;

use redmine_api::RedmineApi;
use redmine_api::time_entries::TimeEntry;

fn main() {
    let redmine = RedmineApi::new(
        "http://localhost:10083".to_string(),
        "9d61c6c2696289c545673daad62272a3ea91f3ef".to_string(),
    );

    let time_entry = TimeEntry {
        issue_id: 1,
        hours: 0.1,
        activity_id: 2,
        comments: "Test am Sonntag".to_string(),
    };

    let result = redmine.time_entries.create(&time_entry);
    println!("Result: {}", result);
}

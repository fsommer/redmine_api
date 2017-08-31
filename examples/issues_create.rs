extern crate redmine_api;

use redmine_api::RedmineApi;
use redmine_api::issues::Issue;

fn main() {
    let redmine = RedmineApi::new(
        "http://localhost:8080".to_string(),
        "bbde69d1999dde8f497199f49bb7b577389b6c0e".to_string(),
    );

    let issue = Issue {
        project_id: 1,
        tracker_id: 1,
        status_id: 1,
        priority_id: 1,
        subject: "New issue",
        description: "This is the description",
        category_id: 0,
        fixed_version_id: 0,
        assigned_to_id: 5,
        parent_issue_id: 1,
        watcher_user_ids: vec![1, 5],
        is_private: true,
        estimated_hours: 1.25,
    };

    let result = redmine.issues().create(&issue);
    println!("Result: {:?}", result);
}

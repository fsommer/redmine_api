extern crate redmine_api;

use redmine_api::RedmineApi;

fn main() {
    let redmine = RedmineApi::new(
        "http://localhost:8080".to_string(),
        "bbde69d1999dde8f497199f49bb7b577389b6c0e".to_string(),
    );

    let result = redmine.time_entries().list().user_id(1).execute().unwrap();
    for item in result {
        println!("ID: {:?}", item.id);
    }
}

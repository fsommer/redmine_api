extern crate redmine_api;

use redmine_api::RedmineApi;

fn main() {
    let redmine = RedmineApi::new(
        "http://localhost:8080".to_string(),
        "bbde69d1999dde8f497199f49bb7b577389b6c0e".to_string(),
    );

    let result = redmine.time_entries().create(1, 0.2, 4)
        .comments("Hello World")
        .spent_on("2017-08-17")
        .execute();
    println!("Result: {:?}", result);
}

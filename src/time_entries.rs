use super::RedmineClient;

pub struct Api {
    client: RedmineClient,
}
impl Api {
    pub fn new(client: RedmineClient) -> Api {
        Api {
            client: client,
        }
    }

    pub fn create(&self, time_entry: &TimeEntry) -> bool {
        let object = format!(
            "{{\n
                \"time_entry\": {{\n
                    \"issue_id\": {},\n
                    \"hours\": {},\n
                    \"activity_id\": {},\n
                    \"comments\": \"{}\"\n
                }}\n
            }}",
            time_entry.issue_id,
            time_entry.hours,
            time_entry.activity_id,
            time_entry.comments,
        );
        self.client.create("/time_entries.json", &object)
    }
}

pub struct TimeEntry {
    pub issue_id: u32,
    pub hours: f32,
    pub activity_id: u8,
    pub comments: String,
}

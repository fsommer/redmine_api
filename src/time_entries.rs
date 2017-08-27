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
        self.client.create("/time_entries.json", &CreateTimeEntry {
            time_entry: time_entry
        })
    }
}

#[derive(Serialize)]
pub struct TimeEntry {
    pub issue_id: u32,
    pub hours: f32,
    pub activity_id: u8,
    pub comments: String,
}

#[derive(Serialize)]
struct CreateTimeEntry<'a> {
    time_entry: &'a TimeEntry,
}

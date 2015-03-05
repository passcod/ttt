use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use super::action::Action;
use time::{self, Timespec};

#[derive(Debug)]
pub struct Entry {
  timestamp: Timespec,
  action: Action,
  project: String,
  task: Option<String>,
  notes: Option<String>
}

impl Entry {
  pub fn new(action: Action, project: String, task: Option<String>, notes: Option<String>) -> Entry {
    Entry { timestamp: time::get_time(), action: action, project: project, task: task, notes: notes }
  }
}

impl ToJson for Entry {
  fn to_json(&self) -> Json {
    let mut d = BTreeMap::new();
    d.insert("timestamp".to_string(), Json::I64(self.timestamp.sec));
    d.insert("action".to_string(),    self.action.to_json());
    d.insert("project".to_string(),   self.project.to_json());
    d.insert("task".to_string(),      self.task.to_json());
    d.insert("notes".to_string(),     self.notes.to_json());
    Json::Object(d)
  }
}

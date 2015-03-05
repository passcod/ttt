use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use std::{fs, os};
use std::io::Write;
use std::path::PathBuf;
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

  pub fn write(&self) {
    let homedir = match os::homedir() {
      Some(p) => p,
      None => panic!("Impossible to get your home dir!")
    };

    let mut filepath = PathBuf::new(&homedir);
    filepath.push(".ttt");
    filepath.push("entries");
    
    match fs::create_dir_all(&filepath) {
      Err(e) => panic!("Error creating directory {}: {}", filepath.display(), e),
      _ => {}
    };

    filepath.push(&format!("{}", self.timestamp.sec));
    filepath.set_extension("json");
    
    let mut file = match fs::File::create(&filepath) {
      Ok(f) => f,
      Err(e) => panic!("Error creating file {}: {}", filepath.display(), e)
    };

    match file.write_all(self.to_json().to_string().as_bytes()) {
      Err(e) => panic!("Error writing to file {}: {}", filepath.display(), e),
      _ => {}
    };
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

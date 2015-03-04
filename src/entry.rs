use super::action;

#[derive(Debug)]
pub struct Entry {
  action: action::Action,
  project: String,
  task: Option<String>,
  notes: Option<String>
}

impl Entry {
  pub fn new(action: action::Action, project: String, task: Option<String>, notes: Option<String>) -> Entry {
    Entry { action: action, project: project, task: task, notes: notes }
  }
}

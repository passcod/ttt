use std::collections::BTreeMap;
use std::time::Duration;

#[derive(Debug)]
pub enum Action {
  Start,
  End,
  Estimate(Duration)
}

pub fn from_string(task_str: &str) -> Option<Action> {
  if task_str == "start" || task_str == "begin" {
    Some(Action::Start)
  } else if task_str == "end" || task_str == "stop" {
    Some(Action::End)
  } else if task_str.starts_with("estimate=") {
    match parse_duration(&task_str[9..]) {
  	  Some(t) => Some(Action::Estimate(t)),
  	  None => None
  	}
  } else {
    None
  }
}

fn parse_duration(duration_str: &str) -> Option<Duration> {
  let mut units: BTreeMap<char, u32> = BTreeMap::new();
  units.insert('Y', 60 * 60 * 24 * 365);
  units.insert('M', 60 * 60 * 24 * 30);
  units.insert('w', 60 * 60 * 24 * 7);
  units.insert('d', 60 * 60 * 24);
  units.insert('h', 60 * 60);
  units.insert('m', 60);
  units.insert('s', 1);

  let mut duration = 0u32;
  let mut n = 0u32;
  let mut u: char;
  for c in duration_str.chars() {
    if c.is_digit(10) {
  	  n *= 10;
  	  n += c.to_digit(10).unwrap();
  	} else {
      match units.get(&c) {
        Some(v) => {
          u = c.clone();
          duration += n * v.clone()
        },
        None => return None
      };
  	  n = 0;
  	}
  }

  if n > 0  {
    //TODO
  }

  if duration == 0 {
    None
  } else {
    Some(Duration::seconds(duration as i64))
  }
}

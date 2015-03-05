use rustc_serialize::json::{ToJson, Json};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub enum Action {
  Start,
  End,
  Estimate(Duration)
}

impl Action {
  pub fn from_str(task_str: &str) -> Option<Action> {
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
}

impl ToJson for Action {
  fn to_json(&self) -> Json {
    match *self {
      Action::Start => Json::String("start".to_string()),
      Action::End => Json::String("end".to_string()),
      Action::Estimate(d) => Json::I64(d.num_seconds())
    }
  }
}

fn parse_duration(duration_str: &str) -> Option<Duration> {
  let units: Vec<char> = vec!['Y', 'M', 'w', 'd', 'h', 'm', 's'];
  let mut times: HashMap<char, u32> = HashMap::with_capacity(7);
  times.insert('Y', 60 * 60 * 24 * 365);
  times.insert('M', 60 * 60 * 24 * 30);
  times.insert('w', 60 * 60 * 24 * 7);
  times.insert('d', 60 * 60 * 24);
  times.insert('h', 60 * 60);
  times.insert('m', 60);
  times.insert('s', 1);

  let mut duration = 0u32;
  let mut n = 0u32;
  let mut u = ' ';
  for c in duration_str.chars() {
    if c.is_digit(10) {
      n *= 10;
      n += c.to_digit(10).unwrap();
    } else {
      match times.get(&c) {
        Some(v) => {
          u = c.clone();
          duration += n * v.clone()
        },
        None => return None
      };
      n = 0;
    }
  }

  if n > 0 && u != ' '  {
    let mut i = -1;
    for k in units.iter() {
      i += 1;
      if *k == u {
        break;
      }
    }

    duration += if i == units.len() - 1 {
      n
    } else {
      times.get(&units[i+1]).unwrap() * n
    };
  }

  if duration == 0 {
    None
  } else {
    Some(Duration::seconds(duration as i64))
  }
}

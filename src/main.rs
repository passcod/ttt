#![feature(std_misc,unicode)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate time;

use action::Action;
use entry::Entry;
use rustc_serialize::json::ToJson;
use std::env;

mod action;
mod entry;

fn main() {
  let args = env::args();
  if args.len() < 3 {
    return help();
  }

  let mut argstrs: Vec<String> = Vec::new();
  for a in args {
    argstrs.push(a.clone());
  }
  argstrs.reverse();

  let _ = argstrs.pop(); // $0 is program name
  let project = argstrs.pop().unwrap();
  let action: Action;
  let task: Option<String>;

  let arg = argstrs.pop().unwrap();
  match Action::from_str(&arg[..]) {
    Some(act) => {
      action = act;
      task = None;
    },
    None => {
      task = Some(arg);
      match argstrs.pop() {
        Some(s) => match Action::from_str(&s[..]) {
          Some(act) => {
            action = act;
          },
          None => return help()
        },
        None => return help()
      };
    }
  }

  let notes = if argstrs.len() == 0 {
    None
  } else {
    Some({
      argstrs.reverse();
      let mut niter = argstrs.iter();
      let mut st = niter.next().unwrap().clone();
      for s in niter {
        st.push_str(" ");
        st.push_str(s);
      }
      st
    })
  };

  let entry = Entry::new(action, project, task, notes);
  println!("{}", entry.to_json().to_string());
}

fn help() {
  println!("Usage: ttt <project> [task] <action> [notes...]
  `action' is one of `start', `end', or `estimate=<duration>',
  where <duration> is a human-readable duration expressed in
  shorthand form, e.g. `30m' or `2h' or `3h25'.");
}

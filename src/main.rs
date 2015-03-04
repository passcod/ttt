extern crate time;

use action::Action;
use std::env;
use std::collections::VecDeque;

mod action;

fn main() {
  let mut args = env::args();
  if args.len() < 3 {
    return help();
  }

  let mut argstrs: VecDeque<String> = VecDeque::with_capacity(args.len() - 1);
  let _ = args.next();
  for arg in args {
    argstrs.push_back(arg);
  }

  assert!(argstrs.len() >= 2);
  let project = argstrs.pop_front().unwrap();
  let action: Action;
  let task: Option<String>;

  {
    let arg = argstrs.pop_front().unwrap();
	match action::from_string(arg.as_slice()) {
	  Some(act) => {
	    action = act;
		task = None;
	  },
	  None => {
	    task = Some(arg);
		match argstrs.pop_front() {
		  Some(s) => match action::from_string(s.as_slice()) {
			Some(act) => {
			  action = act;
			},
			None => return help()
		  },
		  None => return help()
		};
	  }
	}
  }

  let (notes, _) = argstrs.as_slices();

  println!("a:{:?} t:{:?} n:{:?}", action, task, notes);
}

fn help() {
  println!("Usage: ttt <project> [task] <action> [notes...]
  `action' is one of `start', `end', or `estimate=<duration>',
  where <duration> is a human-readable duration expressed in
  shorthand form, e.g. `30m' or `2h' or `3h25'.");
}

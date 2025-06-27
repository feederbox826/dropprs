//! dropprs: Drop privileges, setuid and setgid with optional additional groups
//!
//! Usage: dropprs <uid:gid[:addl_gid]> cmd [args]
//! Example: dropprs 1000:1000:1005 id

use nix::unistd::{execvp, setgid, setgroups, setuid, Gid, Uid};
use std::env;
use std::ffi::CString;
use std::process::exit;

// new
fn parse_gid(gid: &str) -> Gid {
  match gid.parse::<u32>() {
    Ok(g) => Gid::from_raw(g),
    Err(_) => {
      eprintln!("Invalid GID '{}'. Only numeric group IDs are supported.", gid);
      exit(2);
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    eprintln!("Usage: dropprs <uid:gid[:addl_gid]> cmd [args]");
    exit(2);
  }

  let userspec = &args[1];
  let mut parts = userspec.split(':');
  // uid parse and check
  let uid = match parts.next().unwrap_or(&"").parse::<u32>() {
    Ok(u) => Uid::from_raw(u),
    Err(_) => {
      eprintln!("Invalid UID '{}'. Only numeric User IDs are supported.", userspec);
      exit(2);
    }
  };
  // gid
  let gid = parse_gid(parts.next().unwrap_or(&""));
  let addl_gid: Vec<Gid> = match parts.next().unwrap_or("") {
    "" => Vec::new(),
    s => s.split(',')
      .filter(|g| !g.is_empty())
      .map(|g| parse_gid(g))
      .collect(),
  };

  let cmd_args: Vec<CString> = args[2..]
    .iter()
    .map(|s| CString::new(s.as_str()).unwrap_or_else(|_| {
      eprintln!("Argument contains null byte: {}", s);
      exit(2);
    }))
    .collect();

  if cmd_args.is_empty() {
    eprintln!("No command provided to execute.");
    exit(2);
  }
  // clear additional groups
  setgroups(&[]).expect("Failed to clear supplementary groups");

  setgid(gid).expect("Failed to set GID");
  if !addl_gid.is_empty() {
    setgroups(&addl_gid).expect("Failed to set supplementary groups");
  }
  setuid(uid).expect("Failed to set UID");

  match execvp(&cmd_args[0], &cmd_args) {
    Ok(_) => {}
    Err(e) => {
      eprintln!("Failed to exec: {}", e);
      exit(1);
    }
  }
}
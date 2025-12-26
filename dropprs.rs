//! dropprs: Drop privileges, setuid and setgid with optional additional groups
//! not suitable for production use
//!
//! Usage: dropprs <uid:gid[:addl_gid]> cmd [args]
//! Example: dropprs 1000:1000:1005 id

use nix::sys::prctl::set_no_new_privs;
use nix::unistd::{Gid, Uid, execvp, getgid, getuid, setgid, setgroups, setuid};
use std::env;
use std::ffi::CString;
use std::process::exit;

fn parse_id<T>(id: &str, kind: &str, f: impl Fn(u32) -> T) -> T {
    match id.parse::<u32>() {
        Ok(n) => f(n),
        Err(_) => {
            eprintln!("Invalid {} '{}'. Only numeric IDs are supported.", kind, id);
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

    set_no_new_privs().expect("Failed to set no_new_privs");

    let mut parts = args[1].split(':');
    // uid parse and check
    let uid_str = parts.next().unwrap_or("");
    let uid: Uid = parse_id(uid_str, "UID", Uid::from_raw);
    // gid
    let gid_str = parts.next().unwrap_or("");
    let gid: Gid = parse_id(gid_str, "GID", Gid::from_raw);
    // additional gids
    let addl_gid: Vec<Gid> = parts
        .next()
        .unwrap_or("")
        .split(',')
        .filter(|g| !g.is_empty())
        .map(|g| parse_id(g, "additional GID", Gid::from_raw))
        .collect();

    let cmd_args: Vec<CString> = args[2..]
        .iter()
        .map(|s| {
            CString::new(s.as_str()).unwrap_or_else(|_| {
                eprintln!("Argument contains null byte: {}", s);
                exit(2);
            })
        })
        .collect();

    if cmd_args.is_empty() {
        eprintln!("No command provided to execute.");
        exit(2);
    }
    // set groups, gid, uid
    setgroups(&addl_gid).expect("Failed to set supplementary groups");
    setgid(gid).expect("Failed to set GID");
    setuid(uid).expect("Failed to set UID");

    // verify that we dropped privileges
    if getuid() != uid || getgid() != gid {
        eprintln!("Privilege drop failed.");
        exit(1);
    }

    match execvp(&cmd_args[0], &cmd_args) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to exec: {}", e);
            exit(1);
        }
    }
}

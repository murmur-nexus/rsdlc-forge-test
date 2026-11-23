// Shared helper for end-to-end proofs. Every proof runs the real binary.
// Owner: the Plan stage (see AGENTS.md). Builders never edit tests/.
#![allow(dead_code)]

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

pub fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_tally"))
}

/// Run the real program with args and optional stdin. No live model, no network.
pub fn run(args: &[&str], stdin: Option<&str>) -> Output {
    let mut cmd = Command::new(bin());
    cmd.args(args)
        .env_clear()
        // Coverage: the board's ratchet runs the proofs against an instrumented program and needs this one variable.
        .envs(std::env::vars().filter(|(k, _)| k == "LLVM_PROFILE_FILE"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().expect("spawn tally");
    if let Some(s) = stdin {
        // The program may exit before it reads stdin (a bad flag): a broken pipe is then expected, not a failure.
        let _ = child.stdin.as_mut().unwrap().write_all(s.as_bytes());
    }
    drop(child.stdin.take());
    child.wait_with_output().expect("wait tally")
}

pub fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

pub fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

pub fn golden(name: &str) -> String {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(name);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("golden {}: {e}", p.display()))
}

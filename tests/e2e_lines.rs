mod common;
use common::*;

#[test]
fn lines_stdin() {
    let o = run(&["--lines"], Some("a b\nc\n"));
    assert_eq!(stdout(&o), "2\n");
    assert_eq!(o.status.code(), Some(0));
}

#[test]
fn lines_file() {
    let p = std::env::temp_dir().join(format!("tally_lines_{}.txt", std::process::id()));
    std::fs::write(&p, "a\nb\nc\n").unwrap();
    let o = run(&["--lines", p.to_str().unwrap()], None);
    let _ = std::fs::remove_file(&p);
    assert_eq!(stdout(&o), "3\n");
    assert_eq!(o.status.code(), Some(0));
}

#[test]
fn lines_json_error() {
    for args in [["--lines", "--json"], ["--json", "--lines"]] {
        let o = run(&args, Some("a\n"));
        assert_eq!(o.status.code(), Some(2));
        assert_eq!(stdout(&o), "");
        let e = stderr(&o);
        assert!(e.contains("--lines") && e.contains("--json"), "stderr: {e}");
    }
}

#[test]
fn default_unchanged() {
    let o = run(&[], Some("a b\nc\n"));
    assert_eq!(stdout(&o), "2 3 6 -\n");
    assert_eq!(o.status.code(), Some(0));
}

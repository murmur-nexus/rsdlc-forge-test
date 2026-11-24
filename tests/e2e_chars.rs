mod common;
use common::*;

#[test]
fn chars_stdin() {
    let o = run(&["--chars"], Some("héé\n"));
    assert_eq!(stdout(&o), "4\n");
    assert_eq!(o.status.code(), Some(0));
}

#[test]
fn chars_file() {
    let p = std::env::temp_dir().join(format!("tally_chars_{}.txt", std::process::id()));
    std::fs::write(&p, "añb\n").unwrap();
    let o = run(&["--chars", p.to_str().unwrap()], None);
    let _ = std::fs::remove_file(&p);
    assert_eq!(stdout(&o), "4\n");
    assert_eq!(o.status.code(), Some(0));
}

#[test]
fn chars_json_error() {
    for args in [["--chars", "--json"], ["--json", "--chars"]] {
        let o = run(&args, Some("a\n"));
        assert_eq!(o.status.code(), Some(2));
        assert_eq!(stdout(&o), "");
        let e = stderr(&o);
        assert!(e.contains("--chars") && e.contains("--json"), "stderr: {e}");
    }
}

#[test]
fn default_unchanged() {
    let o = run(&[], Some("héé\n"));
    assert_eq!(stdout(&o), "1 1 6 -\n");
    assert_eq!(o.status.code(), Some(0));
}

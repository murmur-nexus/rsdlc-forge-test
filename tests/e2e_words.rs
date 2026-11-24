mod common;
use common::*;

#[test]
fn words_stdin() {
    let o = run(&["--words"], Some("a b\nc\n"));
    assert_eq!(stdout(&o), "3\n");
    assert_eq!(o.status.code(), Some(0));
}

#[test]
fn words_file() {
    let p = std::env::temp_dir().join(format!("tally_words_{}.txt", std::process::id()));
    std::fs::write(&p, "a b\nc d\n").unwrap();
    let o = run(&["--words", p.to_str().unwrap()], None);
    let _ = std::fs::remove_file(&p);
    assert_eq!(stdout(&o), "4\n");
    assert_eq!(o.status.code(), Some(0));
}

#[test]
fn words_json_error() {
    for args in [["--words", "--json"], ["--json", "--words"]] {
        let o = run(&args, Some("a\n"));
        assert_eq!(o.status.code(), Some(2));
        assert_eq!(stdout(&o), "");
        let e = stderr(&o);
        assert!(e.contains("--words") && e.contains("--json"), "stderr: {e}");
    }
}

#[test]
fn default_unchanged() {
    let o = run(&[], Some("a b\nc\n"));
    assert_eq!(stdout(&o), "2 3 6 -\n");
    assert_eq!(o.status.code(), Some(0));
}

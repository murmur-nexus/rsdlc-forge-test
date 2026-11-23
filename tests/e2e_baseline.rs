// Lasting proofs of the baseline behaviour (card 0). They keep running after every later card.
mod common;
use common::*;

#[test]
fn version_output_matches_golden() {
    let o = run(&["--version"], None);
    assert!(o.status.success());
    assert_eq!(stdout(&o), golden("version.txt"));
}

#[test]
fn counts_stdin_lines_words_bytes() {
    let o = run(&[], Some("one two\nthree\n"));
    assert!(o.status.success());
    assert_eq!(stdout(&o), "2 3 14 -\n");
}

#[test]
fn json_output_shape() {
    let o = run(&["--json"], Some("a\n"));
    assert_eq!(
        stdout(&o),
        "{\"file\":\"-\",\"lines\":1,\"words\":1,\"bytes\":2}\n"
    );
}

#[test]
fn missing_file_exits_2_with_message() {
    let o = run(&["/no/such/file"], None);
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("/no/such/file"));
    assert!(stdout(&o).is_empty());
}

#[test]
fn unknown_option_exits_2() {
    let o = run(&["--bogus"], None);
    assert_eq!(o.status.code(), Some(2));
}

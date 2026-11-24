mod common;

#[test]
fn help_prints_usage() {
    let o = common::run(&["--help"], None);
    assert_eq!(o.status.code(), Some(0));
    assert_eq!(
        common::stdout(&o),
        "usage: tally [--json|--lines|--words] [--version] [--help] [FILE...]\nReads stdin when no FILE is given. Prints lines, words and bytes.\n"
    );
    assert_eq!(common::stderr(&o), "");
}

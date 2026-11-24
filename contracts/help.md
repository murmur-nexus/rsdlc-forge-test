# Help: --help

| id | kind | claim | proof | threshold |
|---|---|---|---|---|
| C1 | keep | tally --help prints the usage text on stdout and exits | e2e_help::help_prints_usage | stdout is exactly "usage: tally [--json or --lines or --words] [--version] [--help] [FILE...]\nReads stdin when no FILE is given. Prints lines, words and bytes.\n", stderr empty, exit 0 |

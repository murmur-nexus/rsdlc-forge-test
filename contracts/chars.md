# Chars only: --chars

| id | kind | claim | proof | threshold |
|---|---|---|---|---|
| C1 | new | tally --chars on stdin prints characters, not bytes | e2e_chars::chars_stdin | input "héé\n" gives stdout exactly "4\n", exit 0 |
| C2 | new | tally --chars FILE prints only the bare number, no file name | e2e_chars::chars_file | stdout is exactly "4\n", exit 0 |
| C3 | new | --chars with --json is an error, in either order | e2e_chars::chars_json_error | exit 2, empty stdout, stderr mentions --chars and --json |
| C4 | keep | without --chars the default output is unchanged | e2e_chars::default_unchanged | stdout is "1 1 6 -\n" for "héé\n", exit 0 |

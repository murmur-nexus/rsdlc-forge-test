# Lines only: --lines

| id | kind | claim | proof | threshold |
|---|---|---|---|---|
| C1 | new | tally --lines on stdin prints only the line count | e2e_lines::lines_stdin | stdout is exactly "2\n", exit 0 |
| C2 | new | tally --lines FILE prints only the bare number, no file name | e2e_lines::lines_file | stdout is exactly "3\n", exit 0 |
| C3 | new | --lines with --json is an error, in either order | e2e_lines::lines_json_error | exit 2, empty stdout, stderr mentions --lines and --json |
| C4 | keep | without --lines the default output is unchanged | e2e_lines::default_unchanged | stdout is "2 3 6 -\n", exit 0 |

# Words only: --words

| id | kind | claim | proof | threshold |
|---|---|---|---|---|
| C1 | new | tally --words on stdin prints only the word count | e2e_words::words_stdin | stdout is exactly "3\n", exit 0 |
| C2 | new | tally --words FILE prints only the bare number, no file name | e2e_words::words_file | stdout is exactly "4\n", exit 0 |
| C3 | new | --words with --json is an error, in either order | e2e_words::words_json_error | exit 2, empty stdout, stderr mentions --words and --json |
| C4 | keep | without --words the default output is unchanged | e2e_words::default_unchanged | stdout is "2 3 6 -\n", exit 0 |

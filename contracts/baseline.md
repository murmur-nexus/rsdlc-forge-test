# Contract: baseline (card 0)

## Claims

| id | kind | claim | proof | threshold |
|----|------|-------|-------|-----------|
| C1 | keep | `tally --version` prints the golden version line | e2e_baseline::version_output_matches_golden | stdout equals tests/golden/version.txt |
| C2 | keep | stdin is counted as lines, words, bytes | e2e_baseline::counts_stdin_lines_words_bytes | stdout `2 3 14 -` |
| C3 | keep | a missing file exits 2 and names the file on stderr | e2e_baseline::missing_file_exits_2_with_message | exit code 2 |

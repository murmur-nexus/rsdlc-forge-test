use std::io::Read;
use std::process::ExitCode;

const VERSION_LINE: &str = concat!("tally ", env!("CARGO_PKG_VERSION"));

#[derive(Default, Debug, PartialEq)]
struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
}

fn count(data: &[u8]) -> Counts {
    let text = String::from_utf8_lossy(data);
    Counts {
        lines: data.iter().filter(|b| **b == b'\n').count(),
        words: text.split_whitespace().count(),
        bytes: data.len(),
    }
}

fn usage() -> &'static str {
    "usage: tally [--json|--lines|--words] [--version] [--help] [FILE...]\n\
     Reads stdin when no FILE is given. Prints lines, words and bytes."
}

fn main() -> ExitCode {
    let mut json = false;
    let mut lines_only = false;
    let mut words_only = false;
    let mut files: Vec<String> = Vec::new();
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--version" => {
                println!("{VERSION_LINE}");
                return ExitCode::SUCCESS;
            }
            "--help" => {
                println!("{}", usage());
                return ExitCode::SUCCESS;
            }
            "--json" => json = true,
            "--lines" => lines_only = true,
            "--words" => words_only = true,
            a if a.starts_with("--") => {
                eprintln!("tally: unknown option {a}");
                return ExitCode::from(2);
            }
            _ => files.push(arg),
        }
    }

    if json && lines_only {
        eprintln!("tally: --lines cannot be used with --json");
        return ExitCode::from(2);
    }

    if json && words_only {
        eprintln!("tally: --words cannot be used with --json");
        return ExitCode::from(2);
    }

    let mut inputs: Vec<(String, Vec<u8>)> = Vec::new();
    if files.is_empty() {
        let mut buf = Vec::new();
        if let Err(e) = std::io::stdin().read_to_end(&mut buf) {
            eprintln!("tally: cannot read stdin: {e}");
            return ExitCode::from(2);
        }
        inputs.push(("-".to_string(), buf));
    } else {
        for f in &files {
            match std::fs::read(f) {
                Ok(d) => inputs.push((f.clone(), d)),
                Err(e) => {
                    eprintln!("tally: {f}: {e}");
                    return ExitCode::from(2);
                }
            }
        }
    }

    for (name, data) in &inputs {
        let c = count(data);
        if lines_only {
            println!("{}", c.lines);
        } else if words_only {
            println!("{}", c.words);
        } else if json {
            println!(
                "{{\"file\":\"{}\",\"lines\":{},\"words\":{},\"bytes\":{}}}",
                name, c.lines, c.words, c.bytes
            );
        } else {
            println!("{} {} {} {}", c.lines, c.words, c.bytes, name);
        }
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_simple_text() {
        assert_eq!(
            count(b"a b\nc\n"),
            Counts {
                lines: 2,
                words: 3,
                bytes: 6
            }
        );
    }
}

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Parser;
use owo_colors::OwoColorize;
use regex::{Regex, RegexBuilder};

/// A tiny grep-like CLI.
#[derive(Parser, Debug)]
#[command(name = "naan", version, about, long_about = None)]
struct Cli {
    /// Literal pattern (default) or regex (with --regex)
    pattern: String,

    /// Path of the file to search
    path: PathBuf,

    /// Case-insensitive search
    #[arg(short, long)]
    ignore_case: bool,

    /// Treat PATTERN as a regular expression
    #[arg(short, long)]
    regex: bool,

    /// Print line numbers
    #[arg(short = 'n', long, default_value_t = true, action = clap::ArgAction::Set)]
    line_number: bool,

    /// Print only the number of matching lines
    #[arg(short, long)]
    count: bool,

    /// Select non-matching lines
    #[arg(short = 'v', long)]
    invert_match: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if !args.path.exists() {
        bail!("file not found: {}", args.path.display());
    }

    let file = File::open(&args.path)
        .with_context(|| format!("failed to open file: {}", args.path.display()))?;

    let matcher = build_matcher(&args)?;

    let reader = BufReader::new(file);
    let mut match_count: usize = 0;

    for (idx, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("failed reading line {}", idx + 1))?;

        let is_match = matcher.is_match(&line);
        let selected = if args.invert_match { !is_match } else { is_match };

        if selected {
            match_count += 1;
            if !args.count {
                print_line(&args, idx + 1, &line, &matcher);
            }
        }
    }

    if args.count {
        println!("{}", match_count);
    }

    Ok(())
}

fn build_matcher(args: &Cli) -> Result<Regex> {
    if args.regex {
        let mut b = RegexBuilder::new(&args.pattern);
        b.case_insensitive(args.ignore_case);
        return Ok(b.build().context("invalid regex pattern")?);
    }

    // Literal matching:
    // Escape the string and search as a regex. This keeps one matcher path.
    let literal = regex::escape(&args.pattern);
    let mut b = RegexBuilder::new(&literal);
    b.case_insensitive(args.ignore_case);
    Ok(b.build().expect("escaped literal should always compile"))
}

fn print_line(args: &Cli, line_no: usize, line: &str, matcher: &Regex) {
    // Highlight the first match only (simple + readable).
    let mut out = line.to_string();
    if let Some(m) = matcher.find(line) {
        let (a, b, c) = (&line[..m.start()], &line[m.start()..m.end()], &line[m.end()..]);
        out = format!("{}{}{}", a, b.bright_yellow().bold(), c);
    }

    if args.line_number {
        let ln = format!("{}", line_no).dimmed();
        println!("{}: {}", ln, out);
    } else {
        println!("{}", out);
    }
}

use regex::Regex;
use std::io::stdin;

fn main() {
    let mut line_buffer = String::new();
    let mut bytes_read = read_stdin_line_or_panic(&mut line_buffer);

    let mut maybe_summary: Option<ClippySummary> = None;
    while bytes_read > 0 {
        print!("{line_buffer}");

        if let Some(summary) = maybe_summary {
            if let Some(reference) = maybe_parse_clippy_reference(&line_buffer) {
                print_annotation_clippy_finding(&summary, &reference);
            }
            maybe_summary = None;
        } else {
            maybe_summary = maybe_parse_clippy_summary(&line_buffer);
        }
        line_buffer.clear();
        bytes_read = read_stdin_line_or_panic(&mut line_buffer)
    }
}

fn read_stdin_line_or_panic(line_buffer: &mut String) -> usize {
    stdin().read_line(line_buffer).expect("Error reading stdin")
}

#[derive(Debug)]
struct ClippySummary {
    level: String,
    message: String,
}

fn maybe_parse_clippy_summary(line: &str) -> Option<ClippySummary> {
    let summary_regex = Regex::new("^(warning): (.+)").expect("Invalid regex");
    let (_, [level, message]) = summary_regex.captures(line)?.extract::<2>();
    Some(ClippySummary {
        level: level.to_owned(),
        message: message.trim_end().to_owned(),
    })
}

#[derive(Debug)]
struct ClippyReference {
    file_name: String,
    line_number: u64,
    column_number: u64,
}

fn maybe_parse_clippy_reference(line: &str) -> Option<ClippyReference> {
    let reference_regex = Regex::new(r"^ {2}--> (.+):(\d+):(\d+)").expect("Invalid regex");
    let (_, [file_name, line_number, column_number]) =
        reference_regex.captures(line)?.extract::<3>();
    Some(ClippyReference {
        file_name: file_name.to_owned(),
        line_number: line_number.parse().ok()?,
        column_number: column_number.trim_end().parse().ok()?,
    })
}

fn print_annotation_clippy_finding(summary: &ClippySummary, reference: &ClippyReference) {
    println!(
        "::{} file={},line={},col={}::{}",
        summary.level,
        reference.file_name,
        reference.line_number,
        reference.column_number,
        summary.message
    );
}

use std::ops::Range;

use super::*;

pub fn prettyprint(src: &SrcFile, errors: &[Error]) -> String {
    let mut output = src.str().to_string();

    apply_colors(src, errors, &mut output);
    apply_line_modifications(src, errors, &mut output);

    output.insert_str(0, &format!("- {} errors\n", errors.len()));
    output.insert_str(0, &format!("file {}:\n", src.id()));

    output
}

fn apply_colors(src: &SrcFile, errors: &[Error], output: &mut String) {
    let mut error_spans = errors
        .iter()
        .map(|error| error.span())
        .filter(|span| span.file_id() == src.id())
        .collect::<Vec<Span>>();

    error_spans.sort();

    for i in (1..error_spans.len()).rev() {
        let span = error_spans[i];
        let previus_span = error_spans[i - 1];

        if span.start() <= previus_span.end() {
            error_spans[i - 1] = span.join(previus_span);
            error_spans.remove(i);
        }
    }

    for span in error_spans.into_iter().rev() {
        output.insert_str(span.end(), "\x1b[0m");
        output.insert_str(span.start(), "\x1b[31m");
    }
}

fn apply_line_modifications(src: &SrcFile, errors: &[Error], output: &mut String) {
    output.push('\n');
    let newline_indicies = output
        .char_indices()
        .filter(|(_, c)| *c == '\n')
        .map(|(idx, _)| idx)
        .collect::<Box<[usize]>>();

    let lines = (0..newline_indicies.len())
        .map(|index| {
            let line_start = if index > 0 {
                newline_indicies[index - 1] + 1
            } else {
                0
            };
            let line_end = newline_indicies[index];

            line_start..line_end
        })
        .collect::<Box<[Range<usize>]>>();

    let src_line_digit_count = (src.str().chars().filter(|c| *c == '\n').count() + 1).ilog10();

    for line_index in (0..lines.len()).rev() {
        let line = &lines[line_index];

        let line_number = line_index + 1;
        let line_number_str = format!(
            "{}{}",
            line_number.to_string(),
            " ".repeat((src_line_digit_count - line_number.ilog10()) as usize)
        );

        let trimmed_line = output[line.clone()].trim_end();

        let line_ends_as_red = output[0..line.end]
            .rfind("\x1b[31m")
            .map_or(false, |last_red| {
                output[0..line.end]
                    .rfind("\x1b[0m")
                    .map_or(true, |last_reset| last_red > last_reset)
            });

        let error_messages = {
            let mut error_messages = errors
                .iter()
                .filter(|error| {
                    error.span().file_id() == src.id()
                        && src.line_number(error.span().end()) == line_number
                })
                .map(|error| format!("\x1b[31m   {error}"))
                .collect::<String>();

            if !line_ends_as_red {
                error_messages.push_str("\x1b[0m");
            }

            error_messages
        };

        output.replace_range(
            line.clone(),
            &format!("{line_number_str}  {trimmed_line}{error_messages}"),
        );
    }
}

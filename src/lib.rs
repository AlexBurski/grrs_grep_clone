use std::io::BufRead;
use rayon::prelude::*;

/// Checks if a line contains the given pattern.
///
/// # Arguments
///
/// * `line` - The line to check
/// * `pattern` - The pattern to search for
/// * `ignore_case` - Whether to perform a case-insensitive search
///
/// # Returns
///
/// `true` if the pattern is found in the line, `false` otherwise
pub fn match_line(line: &str, pattern: &str, ignore_case: bool) -> bool {
    if ignore_case {
        line.to_lowercase().contains(&pattern.to_lowercase())
    } else {
        line.contains(pattern)
    }
}

/// Performs a sequential search for a pattern in the given reader.
///
/// # Arguments
///
/// * `reader` - The reader to search in
/// * `pattern` - The pattern to search for
/// * `ignore_case` - Whether to perform a case-insensitive search
/// * `output` - A function to call with each matching line
///
/// # Returns
///
/// A `Result` indicating success or an I/O error
pub fn search_sequential<R: BufRead, F: FnMut(&str)>(
    reader: R,
    pattern: &str,
    ignore_case: bool,
    mut output: F,
) -> std::io::Result<()> {
    for line in reader.lines() {
        let line = line?;
        if match_line(&line, pattern, ignore_case) {
            output(&line);
        }
    }
    Ok(())
}

/// Performs a parallel search for a pattern in the given reader.
///
/// # Arguments
///
/// * `reader` - The reader to search in
/// * `pattern` - The pattern to search for
/// * `ignore_case` - Whether to perform a case-insensitive search
/// * `output` - A function to call with each matching line
///
/// # Returns
///
/// A `Result` indicating success or an I/O error
pub fn search_parallel<R: BufRead, F: Fn(&str) + Sync + Send>(
    reader: R,
    pattern: &str,
    ignore_case: bool,
    output: F,
) -> std::io::Result<()> {
    reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .par_iter()
        .try_for_each(|line| -> std::io::Result<()> {
            if match_line(line, pattern, ignore_case) {
                output(line);
            }
            Ok(())
        })
}

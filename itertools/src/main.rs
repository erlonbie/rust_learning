#![allow(deprecated)]
#![allow(dead_code)]
#![allow(unused_variables)]

use chrono::prelude::*;
#[allow(deprecated)]
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use thiserror::Error;


/// Defines the possible errors that can occur when parsing a log in a specific format.
/// 
/// This enum is used in conjunction with the `Error` derive macro from the `thiserror` crate
/// to provide convenient error handling and reporting with the `Result` type.
/// 
/// The enum has three variants, each representing a distinct kind of parse error:
/// 
/// - `FormatInvalid`: The log line has an invalid format, either because it's missing required
///   elements or because it contains unexpected characters.
/// - `TimestampInvalid`: The timestamp in the log line is invalid, either because it doesn't match
///   the expected format or because it's outside the allowed range.
/// - `InvalidId`: The id field in the log line is invalid, either because it's missing or because
///   it contains an unexpected value.
/// 
/// Usage example:
/// 
/// ```
/// use thiserror::Error;
///
/// #[derive(Debug, Error)]
/// enum ParseMyLogError {
///     #[error("invalid format")]
///     FormatInvalid,
///     #[error("invalid timestamp")]
///     TimestampInvalid,
///     #[error("invalid id")]
///     InvalidId,
/// }
///
/// fn parse_my_log_line(line: &str) -> Result<(), ParseMyLogError> {
///     // Parse the line and return an error if it doesn't conform to the expected format.
///     // Use the `From` trait to convert any parsing errors to `ParseMyLogError`.
///     // ...
/// #   Ok(())
/// }
/// ```
/// 
/// For more information, see the `Error` derive macro documentation in the `thiserror` crate.
#[derive(Debug, Error)]
enum ParseMyLogError {
    #[error("invalid format")]
    FormatInvalid,
    #[error("invalid timestamp")]
    TimestampInvalid,
    #[error("invalid id")]
    InvalidId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord)]
struct MyLog {
    timestamp: DateTime<FixedOffset>,
    id: i32,
    name: String,
}

// impl Ord for MyLog {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.timestamp.cmp(&other.timestamp)
//     }
// }

impl PartialOrd for MyLog {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl TryFrom<&str> for MyLog {
    type Error = ParseMyLogError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut parts = line.split_whitespace();

        // let id = parts
        //     .next()
        //     .ok_or(ParseMyLogError::FormatInvalid)?
        //     .parse()
        //     .map_err(|_| ParseMyLogError::InvalidId)?;
        let id1 = parts.next();
        let id2 = id1.ok_or(ParseMyLogError::InvalidId)?;
        let id3 = id2.parse::<i32>();
        let id = id3.map_err(|_| ParseMyLogError::InvalidId)?;

        let name = parts
            .next()
            .ok_or(ParseMyLogError::FormatInvalid)?
            .to_string();

        let date_str = parts.next().ok_or(ParseMyLogError::FormatInvalid)?;
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| ParseMyLogError::TimestampInvalid)?;
        let offset = FixedOffset::east(0);
        let timestamp = offset.from_utc_date(&date).and_hms(0, 0, 0);

        Ok(Self {
            id,
            name,
            timestamp,
        })
    }
}


/// Reads the lines from the file specified by the given filename.
///
/// # Arguments
///
/// * `filename` - A path to the file to read.
///
/// # Returns
///
/// Returns an `io::Result` containing an `io::Lines<io::BufReader<File>>` that can be
/// iterated over to get each line in the file. If an error occurs while reading the file,
/// an `io::Error` will be returned instead.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::{self, BufRead};
///
/// fn main() -> io::Result<()> {
///     let mut lines = read_lines("file.txt")?;
///     for line in lines {
///         println!("{}", line?);
///     }
///     Ok(())
/// }
///
/// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File
/// where
///     P: AsRef<Path>,
/// {
///     let file = File::open(filename)?;
///     Ok(io::BufReader::new(file).lines())
/// }
/// ```
///
/// This function uses `File::open` to open the file at the given `filename `, and then
/// wraps the resulting file handle in a `BufReader` to read it more efficiently. It then
/// calls the `lines` method on the `BufReader` and returns the resulting iterator over
/// the lines in the file as an `io::Lines<io::BufReader<File>>`.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn main() {
    let log1 = read_lines("./log1.txt").unwrap();
    let log2 = read_lines("./log2.txt").unwrap();

    let log1 = log1.filter_map(|l| MyLog::try_from(l.ok()?.as_ref()).ok());
    let log2 = log2.filter_map(|l| MyLog::try_from(l.ok()?.as_ref()).ok());

    let log_final = log1
        .merge(log2)
        .unique()
        .sorted()
        .collect_vec();

    let mut output = File::create("output.txt").expect("Could not create file!");

    for l in log_final {
        println!("{l:#?}");
        let line = format!("{} {} {}\n", l.id, l.name, l.timestamp);
        output.write_all(line.as_bytes()).expect("Could not write to the file");
    }

    // let user_input = vec!["8.8", "9", "ten", "8.70"];
    // let mut result = vec![];
    // let actual_numbers = user_input.iter().filter(|n| n.parse::<f32>().is_ok());
    // let actual_numbers = user_input.iter().filter_map(|n| n.parse::<f32>().ok());
    // let actual_numbers = user_input.iter().filter_map(|n| n.parse::<f32>().ok()).collect::<Vec<f32>>();
    // let actual_numbers = user_input.iter().for_each(|n| result.push(n.parse::<f32>()));
    //
    // println!("{user_input:?}");
    // println!("{result:?}");
}

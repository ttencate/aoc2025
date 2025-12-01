use std::error::Error;
use std::fs;
use std::io::BufRead;
use std::io;
use std::path::Path;
use std::time::Instant;

use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;

pub use aoc_proc_macros::*;

/// Generates a `fn main()` implementation. Takes one argument, the run function, which should
/// accept a string reference to the input and return the puzzle's output. Example:
///
/// ```
/// fn run(input: &str) -> (u64, u64) {
///     // ...
/// }
///
/// aoc::main!(run);
/// ```
///
/// This needs to be a macro so that it can determine the year from the `CARGO_PKG_NAME` and
/// `CARGO_BIN_NAME` environment variables, set by Cargo during compilation of the main crate.
#[macro_export]
macro_rules! main {
    ($run_fn:expr) => {
        fn main() {
            $crate::main($crate::year!(), $crate::day!(), $run_fn);
        }
    }
}

#[macro_export]
macro_rules! input {
    () => {
        $crate::input($crate::year!(), $crate::day!())
    }
}

#[macro_export]
macro_rules! example {
    ($idx:expr) => {
        $crate::example($crate::year!(), $crate::day!(), $idx)
    }
}

pub trait Answer {
    fn show(&self) -> String;
}

impl Answer for usize {
    fn show(&self) -> String {
        self.to_string()
    }
}

impl Answer for u64 {
    fn show(&self) -> String {
        self.to_string()
    }
}

impl Answer for i64 {
    fn show(&self) -> String {
        self.to_string()
    }
}

impl Answer for String {
    fn show(&self) -> String {
        if self.contains('\n') {
            "\n".to_owned() + self
        } else {
            self.to_owned()
        }
    }
}

impl<T, U> Answer for (T, U) where T: Answer, U: Answer {
    fn show(&self) -> String {
        format!("Part 1: {}\nPart 2: {}", self.0.show(), self.1.show())
    }
}

pub fn main<A: Answer, F: FnOnce(&str) -> A>(year: u32, day: u32, run_fn: F) {
    let input = input(year, day);

    let start = Instant::now();
    let answer = run_fn(&input);
    let duration = start.elapsed();

    println!(
        "Answer to {} day {} ({}.{:03} s):\n{}",
        year, day, duration.as_secs(), duration.subsec_millis(), answer.show());
}

pub fn input(year: u32, day: u32) -> String {
    let input_file_name = input_file_name(year, day);
    fs::read_to_string(&input_file_name)
        .or_else(|_err| -> Result<String, Box<dyn Error>> {
            println!("Input file {} could not be read, fetching...", input_file_name);
            let contents = fetch_input(year, day)?;
            ensure_dir_exists(&input_file_name)?;
            fs::write(&input_file_name, &contents)?;
            Ok(contents)
        })
        .unwrap()
}

pub fn example(year: u32, day: u32, index: usize) -> String {
    let example_file_name = example_file_name(year, day, index);
    fs::read_to_string(&example_file_name)
        .or_else(|_err| -> Result<String, Box<dyn Error>> {
            println!("Example file {} could not be read, fetching...", example_file_name);
            let num_examples = fetch_examples(year, day)?;
            if index >= num_examples {
                panic!(
                    "tried to read example {} but there are only {} examples for year {}, day {}",
                    index, num_examples, year, day);
            }
            let contents = fs::read_to_string(&example_file_name)
                .unwrap();
            Ok(contents)
        })
        .unwrap()
}

fn input_file_name(_year: u32, day: u32) -> String {
    format!("inputs/{:02}.in", day)
}

fn example_file_name(_year: u32, day: u32, index: usize) -> String {
    format!("examples/{:02}-{}.example", day, index)
}

fn load_session_cookie() -> Result<String, io::Error> {
    let cookie_file_name = ".session_cookie";
    fs::read_to_string(cookie_file_name)
        .map(|s| s.trim().to_string())
        .or_else(|_err| -> Result<String, io::Error> {
            println!("No session cookie found. Please log in to https://adventofcode.com/ in your browser, open the browser console, copy the value of the 'session' cookie, and paste it here:");
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line)?;
            ensure_dir_exists(&cookie_file_name)?;
            fs::write(&cookie_file_name, &line)?;
            Ok(line.trim().to_string())
        })
}

fn fetch_input(year: u32, day: u32) -> Result<String, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    send_get_request(&url)?
        .text()
        .map_err(From::from)
}

fn fetch_examples(year: u32, day: u32) -> Result<usize, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let mut response = send_get_request(&url)?;
    let root = parse_html().from_utf8().read_from(&mut response)?;
    let mut num_examples = 0;
    for (index, node) in root.select("pre").unwrap().enumerate() {
        let file_name = example_file_name(year, day, index);
        ensure_dir_exists(&file_name)?;
        fs::write(&file_name, node.text_contents())?;
        num_examples += 1;
    }
    Ok(num_examples)
}

fn send_get_request(url: &str) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent("aoc2022 by ttencate@gmail.com, https://github.com/ttencate/aoc2022.git")
        .build()
        .expect("failed to build client");
    let session_cookie = load_session_cookie()?;
    let response = client.get(url)
        .header(reqwest::header::COOKIE, format!("session={}", session_cookie))
        .send()
        .expect("request failed")
        .error_for_status()?;
    Ok(response)
}

fn ensure_dir_exists(file_name: &str) -> Result<(), io::Error> {
    if let Some(parent) = Path::new(file_name).parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

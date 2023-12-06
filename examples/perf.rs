use std::{fs, time::Instant};
use tnil::prelude::*;

fn main() {
    let mut path = env!("CARGO_MANIFEST_DIR").to_owned();
    path += "/examples/perf.txt";
    let contents = fs::read_to_string(path).expect("the file should be there");

    let mut successes: usize = 0;
    let mut failures: usize = 0;

    let start = Instant::now();

    for word in contents.lines() {
        match word.parse::<word::ShortcutCheckedFormative>() {
            Ok(_) => {
                successes += 1;
            }
            Err(error) => {
                failures += 1;
                eprintln!("{word:<30} {error:<20?}: {error}");
            }
        }
    }

    let end = Instant::now();

    let diff = end.checked_duration_since(start).unwrap_or_default();

    eprintln!("{diff:?} passed");
    eprintln!("{successes:?} succeeded, {failures:?} failed");

    if failures > 0 {
        panic!();
    }
}

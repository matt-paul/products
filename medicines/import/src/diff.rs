use crate::csv;
use indicatif::{HumanDuration, ProgressBar};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str,
    time::Instant,
};

pub fn generate_diff(file1_path: &str, file2_path: &str) {
    let file1_path_csv = format!("{}.csv", file1_path);
    let file1 = File::open(&file1_path_csv).unwrap();
    let file1_reader = BufReader::new(file1);
    let file1_lines = file1_reader.lines();
    let file1_lines_unwrapped = file1_lines.map(|line_result| line_result.unwrap());
    println!("File1 loaded");

    let file2_path_csv = format!("{}.csv", file2_path);
    let file2 = File::open(file2_path_csv).unwrap();
    let file2_reader = BufReader::new(file2);
    let file2_lines = file2_reader.lines();
    let mut i = 0;
    let file2_lines_unwrapped = file2_lines
        .map(|line_result| {
            i += 1;
            line_result.unwrap()
        })
        .collect::<String>();
    println!("File2 loaded");

    let mut deleted_files: Vec<String> = vec![];

    let started = Instant::now();
    let progress_bar = ProgressBar::new(i as u64);

    for (_index, file1_line) in file1_lines_unwrapped.enumerate() {
        if !file2_lines_unwrapped.contains(&file1_line) {
            println!("Deleted line: {}", &file1_line);
            deleted_files.push(file1_line);
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();
    println!(
        "Working out diff took: {}",
        HumanDuration(started.elapsed())
    );
    csv::write_csv(file1_path, deleted_files);
}

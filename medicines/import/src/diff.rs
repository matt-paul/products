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
    let file1_lines_unwrapped = file1_lines
        .map(|line_result| line_result.unwrap())
        .collect::<Vec<String>>();
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
        .collect::<Vec<String>>();
    println!("File2 loaded");

    let mut deleted_files: Vec<String> = vec![];

    let started = Instant::now();
    let progress_bar = ProgressBar::new(i as u64);

    for file1_line in &file1_lines_unwrapped {
        if !file2_lines_unwrapped.contains(&file1_line) {
            println!("Deleted line: {}", &file1_line);
            deleted_files.push(file1_line.to_string());
        }
        progress_bar.inc(1);
    }
    println!(
        "Working out deleted files took: {}",
        HumanDuration(started.elapsed())
    );
    csv::write_lines_to_csv(&format!("deleted_{}", file1_path), &deleted_files);
    progress_bar.set_position(0);
    let mut new_files: Vec<String> = vec![];

    for file2_line in &file2_lines_unwrapped {
        if !file1_lines_unwrapped.contains(&file2_line) {
            println!("New line: {}", &file2_line);
            new_files.push(file2_line.to_string());
        }
        progress_bar.inc(1);
    }

    println!(
        "Working out new files took: {}",
        HumanDuration(started.elapsed())
    );
    csv::write_lines_to_csv(&format!("new_{}", file1_path), &new_files);

    progress_bar.finish();
}

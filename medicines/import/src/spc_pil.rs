use crate::{csv, hash::hash, index_manager, metadata, model, pdf, report::Report, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use indicatif::{HumanDuration, ProgressBar};
use std::{
    collections::HashMap,
    fs,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str,
    time::Instant,
};
use tokio_core::reactor::Core;

// TODO: identify updates as well as delete and new
// TODO: do the same delete route for pars? 
// TODO: also insert new to the index?

pub fn import(
    dir: &Path,
    client: Client,
    mut core: Core,
    verbosity: i8,
    dryrun: bool,
) -> Result<(), AzureError> {
    if let Ok(records) = csv::load_csv_with_autodetect(dir) {
        if dryrun {
            println!("This is a dry run, nothing will be uploaded!");
        }
        let started = Instant::now();
        let mut report = Report::new(verbosity);
        let pdfs = pdf::get_pdfs(dir).expect("Could not load any PDFs.");
        let progress_bar = ProgressBar::new(pdfs.len() as u64);
        for path in pdfs {
            let key = path
                .file_stem()
                .expect("file has no stem")
                .to_str()
                .unwrap();
            if let Some(record) = records.get(&key.to_lowercase()) {
                let mut metadata: HashMap<&str, &str> = HashMap::new();

                let file_name = metadata::sanitize(&record.filename);
                metadata.insert("file_name", &file_name);

                let release_state = metadata::sanitize(&record.release_state);
                metadata.insert("release_state", &release_state);

                if release_state != "Y" {
                    report.add_skipped_unreleased(&file_name, &release_state);
                    continue;
                }

                let doc_type = format!(
                    "{:?}",
                    match record.second_level.as_ref() {
                        "PIL" => model::DocType::Pil,
                        "SPC" => model::DocType::Spc,
                        _ => panic!("unexpected doc type"),
                    }
                );
                metadata.insert("doc_type", &doc_type);

                let title = metadata::sanitize(&record.title);
                metadata.insert("title", &title);

                let pl_numbers = metadata::extract_product_licences(&title);
                metadata.insert("pl_number", &pl_numbers);

                let rev_label = metadata::sanitize(&record.rev_label);
                metadata.insert("rev_label", &rev_label);

                let created = record.created.to_rfc3339();
                metadata.insert("created", &created);

                let product_name = metadata::sanitize(&record.product_name);
                metadata.insert("product_name", &product_name);

                let active_substances = metadata::to_array(&record.substance_name);
                let substance_name = metadata::to_json(active_substances.clone());
                metadata.insert("substance_name", &substance_name);

                let facets = metadata::to_json(metadata::create_facets_by_active_substance(
                    &product_name,
                    active_substances,
                ));
                metadata.insert("facets", &facets);

                let file_data = fs::read(path)?;
                let hash = hash(&file_data);

                if (report).already_uploaded_file_with_hash(&hash) {
                    report.add_skipped_duplicate(&file_name, &hash);
                    continue;
                }

                if !dryrun {
                    storage::upload(&hash, &client, &mut core, &file_data, &metadata, verbosity)?;
                }
                report.add_uploaded(&file_name, &hash, &pl_numbers);
            } else {
                report.add_skipped_incomplete(key);
            }
            if verbosity == 0 {
                progress_bar.inc(1);
            }
        }
        progress_bar.finish();
        println!(
            "Importing SPCs & PILs finished in {}",
            HumanDuration(started.elapsed())
        );
        report.print_report();
    }
    Ok(())
}

fn get_file_lines(file_path: &str) -> Vec<String> {
    let file_path_csv = format!("{}.csv", file_path);
    let file = File::open(&file_path_csv).unwrap();
    let file_reader = BufReader::new(file);
    let file_lines = file_reader.lines();
    file_lines
        .map(|line_result| line_result.unwrap())
        .collect::<Vec<String>>()
}

pub fn generate_diff(file1_path: &str, file2_path: &str) {
    let file1_lines_unwrapped = get_file_lines(file1_path);
    let file2_lines_unwrapped = get_file_lines(file2_path);

    // include header line
    let mut deleted_files: Vec<String> = vec![file1_lines_unwrapped[0].to_string()];

    let started = Instant::now();
    let progress_bar = ProgressBar::new(file2_lines_unwrapped.len() as u64);

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
    // include header line
    let mut new_files: Vec<String> = vec![file1_lines_unwrapped[0].to_string()];

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

pub fn delete(
    dir: &Path,
    delete_file: &str,
    client: Client,
    mut core: Core,
    verbosity: i8,
    dryrun: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(delete_records) = csv::load_csv(&format!("{}.csv", delete_file)) {
        if dryrun {
            println!("This is a dry run, nothing will be deleted!");
        } else {
            storage::create_container(&client, &mut core, verbosity)?;
        }
        let started = Instant::now();
        let mut report = Report::new(verbosity);
        let pdfs = pdf::get_pdfs(dir).expect("Could not load any PDFs.");
        let progress_bar = ProgressBar::new(pdfs.len() as u64);
        for path in pdfs {
            let key = path
                .file_stem()
                .expect("file has no stem")
                .to_str()
                .unwrap();
            if let Some(record) = delete_records.get(&key.to_lowercase()) {
                println!("Key found: {}", &key.to_lowercase());
                let file_data = fs::read(&path)?;
                let hash = hash(&file_data);
                let file_name = metadata::sanitize(&record.filename);

                if !dryrun {
                    let success = index_manager::delete(&hash, verbosity)?;
                    if success {
                        report.add_deleted_from_index(&file_name, &hash);
                        storage::delete(&hash, &client, &mut core, verbosity)?;
                        report.add_deleted_from_container(&file_name, &hash);
                    } else {
                        report.add_failed_deleted_from_index(&file_name, &hash);
                    }
                }
            }
            if verbosity == 0 {
                progress_bar.inc(1);
            }
        }
        progress_bar.finish();
        println!(
            "Deleting SPCs & PILs finished in {}",
            HumanDuration(started.elapsed())
        );
        report.print_report();
    } else {
        println!("No records loaded");
    }
    Ok(())
}

pub fn upload(
    dir: &Path,
    new_file: &str,
    client: Client,
    mut core: Core,
    verbosity: i8,
    dryrun: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(new_records) = csv::load_csv(&format!("{}.csv", new_file)) {
        if dryrun {
            println!("This is a dry run, nothing will be uploaded!");
        } else {
            storage::create_container(&client, &mut core, verbosity)?;
        }
        let mut report = Report::new(verbosity);
        let started = Instant::now();
        let pdfs = pdf::get_pdfs(dir).expect("Could not load any PDFs.");
        let progress_bar = ProgressBar::new(pdfs.len() as u64);
        for path in pdfs {
            let key = path
                .file_stem()
                .expect("file has no stem")
                .to_str()
                .unwrap();
            if let Some(record) = new_records.get(&key.to_lowercase()) {
                println!("Key found: {}", &key.to_lowercase());
                let mut metadata: HashMap<&str, &str> = HashMap::new();

                let file_name = metadata::sanitize(&record.filename);
                metadata.insert("file_name", &file_name);

                let release_state = metadata::sanitize(&record.release_state);
                metadata.insert("release_state", &release_state);

                if release_state != "Y" {
                    report.add_skipped_unreleased(&file_name, &release_state);
                    continue;
                }

                let doc_type = format!(
                    "{:?}",
                    match record.second_level.as_ref() {
                        "PIL" => model::DocType::Pil,
                        "SPC" => model::DocType::Spc,
                        _ => panic!("unexpected doc type"),
                    }
                );
                metadata.insert("doc_type", &doc_type);

                let title = metadata::sanitize(&record.title);
                metadata.insert("title", &title);

                let pl_numbers = metadata::extract_product_licences(&title);
                metadata.insert("pl_number", &pl_numbers);

                let rev_label = metadata::sanitize(&record.rev_label);
                metadata.insert("rev_label", &rev_label);

                let created = record.created.to_rfc3339();
                metadata.insert("created", &created);

                let product_name = metadata::sanitize(&record.product_name);
                metadata.insert("product_name", &product_name);

                let active_substances = metadata::to_array(&record.substance_name);
                let substance_name = metadata::to_json(active_substances.clone());
                metadata.insert("substance_name", &substance_name);

                let facets = metadata::to_json(metadata::create_facets_by_active_substance(
                    &product_name,
                    active_substances,
                ));
                metadata.insert("facets", &facets);

                let file_data = fs::read(path)?;
                let hash = hash(&file_data);

                if (report).already_uploaded_file_with_hash(&hash) {
                    report.add_skipped_duplicate(&file_name, &hash);
                    continue;
                }

                if !dryrun {
                    storage::upload(&hash, &client, &mut core, &file_data, &metadata, verbosity)?;
                }
                report.add_uploaded(&file_name, &hash, &pl_numbers);
            } else {
            }
            if verbosity == 0 {
                progress_bar.inc(1);
            }
        }
        progress_bar.finish();
        println!(
            "Uploading SPCs & PILs finished in {}",
            HumanDuration(started.elapsed())
        );
        report.print_report();
    } else {
        println!("No records loaded");
    }
    Ok(())
}

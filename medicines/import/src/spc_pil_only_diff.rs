use crate::{csv, hash::hash, index_manager, metadata, model, pdf, report::Report, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use indicatif::{HumanDuration, ProgressBar};
use reqwest;
use std::{collections::HashMap, fs, path::Path, str, time::Instant};
use tokio_core::reactor::Core;

pub fn import(
    dir: &Path,
    delete_file: &str,
    new_upload_file: &str,
    client: Client,
    mut core: Core,
    verbosity: i8,
    dryrun: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(delete_records) = csv::load_csv(&format!("{}.csv", delete_file)) {
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
            if let Some(delete_record) = delete_records.get(&key.to_lowercase()) {
                println!("Key found: {}", &key.to_lowercase());
                let file_data = fs::read(&path)?;
                let hash = hash(&file_data);

                // if (report).already_deleted_file_with_hash(&hash) {
                //     report.add_skipped_duplicate(&file_name, &hash);
                //     continue;
                // }

                if !dryrun {
                    index_manager::delete(&hash, verbosity)?;
                }
            // report.add_uploaded(&file_name, &hash, &pl_numbers);
            } else {
                // report.add_skipped_incomplete(key);
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

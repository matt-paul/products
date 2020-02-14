use crate::{csv, hash::hash, metadata, model, pdf, report::Report, storage};
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::prelude::*;
use indicatif::{HumanDuration, ProgressBar};
use std::{collections::HashMap, fs, path::Path, str, time::Instant, fs::{DirEntry, File},
    io::BufReader};
use tokio_core::reactor::Core;

pub fn generate_diff(
    file1: &str,
    file2: &str
) {
    let file = File::open(file1);
    let lines = io::BufReader::new(file).lines().expect("File1 should load");
}

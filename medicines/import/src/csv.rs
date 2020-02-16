use crate::model::Record;
use csv;
use std::{
    collections::HashMap,
    fs,
    fs::{DirEntry, File, OpenOptions},
    io::{BufReader, Write},
    path::Path,
    writeln,
};

pub fn load_csv(dir: &Path) -> Result<HashMap<String, Record>, std::io::Error> {
    if let Some(Ok(f)) =
        fs::read_dir(dir)?.find(|f| is_csv(f.as_ref().expect("No CSV file found!")))
    {
        println!("Found CSV file: {:?}", f);
        let file = File::open(&f.path())?;
        let mut rdr = csv::Reader::from_reader(BufReader::new(file));
        Ok(rdr
            .deserialize()
            .map(|r: Result<Record, csv::Error>| {
                let r = r.expect("Failed to deserialize");
                (r.filename.clone().to_lowercase(), r)
            })
            .collect::<HashMap<String, Record>>())
    } else {
        panic!("shouldn't get here");
    }
}

pub fn write_csv(path: &str, lines: Vec<String>) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("./deleted_{}.csv", path))
        .unwrap();

    for line in lines {
        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn is_csv(f: &DirEntry) -> bool {
    "csv" == f.path().extension().unwrap_or_default()
}

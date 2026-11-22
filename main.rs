mod record;

use record::SpeciesRecord;
use std::error::Error;
use std::fs::File;
use std::process;

fn main() {
    let file_path = "dataset.csv";
    let records_to_load = 5;

    match load_initial_records(file_path, records_to_load) {
        Ok(records) => {
            println!("Successfully loaded {} records:\n", records.len());
            for record in &records {
                println!("{:#?}", record);
            }
        }
        Err(err) => {
            eprintln!("Startup Error: Could not load '{}'. Reason: {}", file_path, err);
            process::exit(1);
        }
    }
}

fn load_initial_records(path: &str, limit: usize) -> Result<Vec<SpeciesRecord>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut records: Vec<SpeciesRecord> = Vec::with_capacity(limit);

    // .skip(1) bypasses the French header row (row 2)
    // .take(limit) reads only the first `limit` data records
    for result in reader.deserialize().skip(1).take(limit) {
        let record: SpeciesRecord = result?;
        records.push(record);
    }

    Ok(records)
}

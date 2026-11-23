//! Persistence Layer handles stuff for CSV File-I/O and record serialization.
//!
//! respoable  reading records from the dataset and writing
//! modified records out to UUID-named files.

use crate::model::LakeSpeciesRecord;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use uuid::Uuid;

/// Repository handles CSV file reading and UUID-based file writing
pub struct CsvRepository {
    /// Path to the primary input CSV dataset file.
    source_file_path: String,
}

impl CsvRepository {
    /// Creates a new [`CsvRepository`] targeting `source_file_path`.
    pub fn new(source_file_path: &str) -> Self {
        Self {
            source_file_path: source_file_path.to_string(),
        }
    }

    /// Reads up to `limit` records from the CSV dataset.
    ///
    /// Skips the first two header rows and returns all
    /// available rows if the file contains fewer than 100 records.
    ///
    /// # Errors
    /// Returns an error if the file is missing or unreadable
    pub fn fetch_records(&self, limit: usize) -> Result<Vec<LakeSpeciesRecord>, Box<dyn Error>> {
        let file = File::open(&self.source_file_path)?;
        let reader = BufReader::new(file);

        let mut loaded_records = Vec::with_capacity(limit);

        for (line_num, line_result) in reader.lines().enumerate().skip(2) {
            if loaded_records.len() >= limit {
                break;
            }

            let line = line_result?;
            if line.trim().is_empty() {
                continue;
            }

            match Self::parse_csv_line(&line) {
                Ok(record) => loaded_records.push(record),
                Err(e) => {
                    return Err(format!("Malformed CSV data at line {}: {}", line_num + 1, e).into());
                }
            }
        }

        Ok(loaded_records)
    }

    /// Writes `records` to a new CSV file named with a generated v4 UUID
    ///
    /// Includes both the header rows at the top of the file.
    ///
    /// # Returns
    /// The generated output filename.
    ///
    /// # Errors
    /// Returns an error if the output file cannot be created.
    pub fn save_records_with_uuid(&self, records: &[LakeSpeciesRecord]) -> Result<String, Box<dyn Error>> {
        let output_filename = format!("{}.csv", Uuid::new_v4());
        let file = File::create(&output_filename)?;
        let mut writer = BufWriter::new(file);

        writeln!(
            writer,
            "Identification,Lake identification,Year,Species code,Species,Percent abundance"
        )?;
        writeln!(
            writer,
            "Identification ,Identification des lacs,Année,Code de l’espèce,Espèce,Pourcentage de l’abondance"
        )?;

        for record in records {
            writeln!(
                writer,
                "{},{},{},{},{},{}",
                record.identification,
                record.lake_identification,
                record.year,
                record.species_code,
                record.species,
                record.percent_abundance
            )?;
        }

        writer.flush()?;
        Ok(output_filename)
    }

    /// Parses a single comma-separated line into a [`LakeSpeciesRecord`].
    ///
    /// # Errors
    /// Returns an error if any of the six required columns are missing or fail type parsing.
    fn parse_csv_line(line: &str) -> Result<LakeSpeciesRecord, Box<dyn Error>> {
        let mut cols = line.split(',').map(str::trim);

        let identification = cols
            .next()
            .ok_or("Missing Identification column")?
            .parse()?;
        let lake_identification = cols
            .next()
            .ok_or("Missing Lake identification column")?
            .to_string();
        let year = cols
            .next()
            .ok_or("Missing Year column")?
            .parse()?;
        let species_code = cols
            .next()
            .ok_or("Missing Species code column")?
            .to_string();
        let species = cols
            .next()
            .ok_or("Missing Species column")?
            .to_string();
        let percent_abundance = cols
            .next()
            .ok_or("Missing Percent abundance column")?
            .parse()?;

        Ok(LakeSpeciesRecord {
            identification,
            lake_identification,
            year,
            species_code,
            species,
            percent_abundance,
        })
    }
}

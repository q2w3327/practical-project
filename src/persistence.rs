use crate::model::LakeSpeciesRecord;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use uuid::Uuid;

pub struct CsvRepository {
    source_file_path: String,
}

impl CsvRepository {
    pub fn new(source_file_path: &str) -> Self {
        Self {
            source_file_path: source_file_path.to_string(),
        }
    }

    /// Reads up to `limit` records from the CSV file on disk.
    /// If fewer than `limit` records exist in the file, returns all of them.
    pub fn fetch_records(&self, limit: usize) -> Result<Vec<LakeSpeciesRecord>, Box<dyn Error>> {
        let file = File::open(&self.source_file_path)?;
        let reader = BufReader::new(file);

        let mut loaded_records = Vec::with_capacity(limit);

        // Skip the first 2 lines (English header row + French bilingual header row)
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

    /// Persists the in-memory records to a new CSV file named with a generated UUID.
    /// Returns the generated filename on success.
    pub fn save_records_with_uuid(&self, records: &[LakeSpeciesRecord]) -> Result<String, Box<dyn Error>> {
        let output_filename = format!("{}.csv", Uuid::new_v4());
        let file = File::create(&output_filename)?;
        let mut writer = BufWriter::new(file);

        // Write bilingual headers matching the original dataset structure
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

    /// Helper method to map a single CSV row into a LakeSpeciesRecord
    fn parse_csv_line(line: &str) -> Result<LakeSpeciesRecord, Box<dyn Error>> {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 6 {
            return Err(format!("Expected 6 columns, found {}", fields.len()).into());
        }

        Ok(LakeSpeciesRecord {
            identification: fields[0].trim().parse()?,
            lake_identification: fields[1].trim().to_string(),
            year: fields[2].trim().parse()?,
            species_code: fields[3].trim().to_string(),
            species: fields[4].trim().to_string(),
            percent_abundance: fields[5].trim().parse()?,
        })
    }
}
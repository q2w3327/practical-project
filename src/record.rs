use std::error::Error;
use std::fmt;

/// Record / Data-Transfer Object matching the dataset columns
#[derive(Debug, Clone)]
pub struct LakeSpeciesRecord {
    pub identification: u32,
    pub lake_identification: String,
    pub year: u16,
    pub species_code: String,
    pub species: String,
    pub percent_abundance: f64,
}

impl LakeSpeciesRecord {
    /// Parses a single CSV line into a LakeSpeciesRecord
    pub fn from_csv_line(line: &str) -> Result<Self, Box<dyn Error>> {
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

/// Formats the record data cleanly for screen output
impl fmt::Display for LakeSpeciesRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {:<4} | Lake: {:<6} | Year: {} | Code: {:<6} | Species: {:<25} | Abundance: {:.2}%",
            self.identification,
            self.lake_identification,
            self.year,
            self.species_code,
            self.species,
            self.percent_abundance
        )
    }
}

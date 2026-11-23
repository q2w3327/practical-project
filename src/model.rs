use std::fmt;

/// Model / Entity Object matching the dataset columns
#[derive(Debug, Clone)]
pub struct LakeSpeciesRecord {
    pub identification: u32,
    pub lake_identification: String,
    pub year: u16,
    pub species_code: String,
    pub species: String,
    pub percent_abundance: f64,
}

impl fmt::Display for LakeSpeciesRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {:<4} | Lake: {:<6} | Year: {} | Code: {:<6} | Species: {:<32} | Abundance: {:>6.2}%",
            self.identification,
            self.lake_identification,
            self.year,
            self.species_code,
            self.species,
            self.percent_abundance
        )
    }
}

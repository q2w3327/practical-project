//! Model Layer containing core entities for the dataset.
//!
//! Defines [`LakeSpeciesRecord`] object mapped to the columns of the source CSV dataset.

use std::fmt;

/// Represents a single diatom species observation recorded in a lake sample.
///
/// Each field corresponds to a column in the source dataset
#[derive(Debug, Clone)]
pub struct LakeSpeciesRecord {
    /// Unique numeric row identifier (`Identification`).
    pub identification: u32,
    /// Code identifying the sampled lake (`Lake identification`).
    pub lake_identification: String,
    /// Calendar year the sample was collected (`Year`).
    pub year: u16,
    /// Six-letter shorthand taxonomic code (`Species code`).
    pub species_code: String,
    /// Full scientific species name (`Species`).
    pub species: String,
    /// Relative percentage abundance of the species in the sample (`Percent abundance`).
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


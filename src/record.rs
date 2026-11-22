use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpeciesRecord {
    #[serde(rename = "Identification")]
    pub identification: String,

    #[serde(rename = "Lake identification")]
    pub lake_identification: String,

    #[serde(rename = "Year")]
    pub year: u32,

    #[serde(rename = "Species code")]
    pub species_code: String,

    #[serde(rename = "Species")]
    pub species: String,

    #[serde(rename = "Percent abundance")]
    pub percent_abundance: f64,
}

impl SpeciesRecord {

    // Column name constants
    pub const COL_IDENTIFICATION: &'static str = "Identification";
    pub const COL_LAKE_IDENTIFICATION: &'static str = "Lake identification";
    pub const COL_YEAR: &'static str = "Year";
    pub const COL_SPECIES_CODE: &'static str = "Species code";
    pub const COL_SPECIES: &'static str = "Species";
    pub const COL_PERCENT_ABUNDANCE: &'static str = "Percent abundance";

    // Constructor
    pub fn new(
        identification: String,
        lake_identification: String,
        year: u32,
        species_code: String,
        species: String,
        percent_abundance: f64,
    ) -> Self {
        Self {
            identification,
            lake_identification,
            year,
            species_code,
            species,
            percent_abundance,
        }
    }

    // Example Accessor (Getter)
    pub fn percent_abundance(&self) -> f64 {
        self.percent_abundance
    }

    // Example Mutator (Setter with validation)
    pub fn set_percent_abundance(&mut self, value: f64) -> Result<(), &'static str> {
        if (0.0..=100.0).contains(&value) {
            self.percent_abundance = value;
            Ok(())
        } else {
            Err("Percent abundance must be between 0.0 and 100.0")
        }
    }
}

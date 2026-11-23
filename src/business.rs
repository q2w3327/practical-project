use crate::model::LakeSpeciesRecord;
use crate::persistence::CsvRepository;
use std::cmp::Ordering;
use std::error::Error;

/// Represents the dataset columns available for sorting
#[derive(Debug, Clone, Copy)]
pub enum SortColumn {
    Identification,
    LakeIdentification,
    Year,
    SpeciesCode,
    Species,
    PercentAbundance,
}

pub struct LakeSpeciesService {
    /// Sequential data structure stored in memory within the Business Layer
    records: Vec<LakeSpeciesRecord>,
    repository: CsvRepository,
    record_limit: usize,
}

impl LakeSpeciesService {
    pub fn new(repository: CsvRepository, record_limit: usize) -> Self {
        Self {
            records: Vec::new(),
            repository,
            record_limit,
        }
    }

    /// Initializes the in-memory data structure on startup
    pub fn initialize_records(&mut self) -> Result<(), Box<dyn Error>> {
        self.reload_records()
    }

    /// Reloads up to `record_limit` records from disk, replacing in-memory data
    pub fn reload_records(&mut self) -> Result<(), Box<dyn Error>> {
        let fresh_records = self.repository.fetch_records(self.record_limit)?;
        self.records.clear();
        self.records = fresh_records;
        Ok(())
    }

    /// Persists the current in-memory records to a new UUID-named CSV file
    pub fn persist_to_new_csv(&self) -> Result<String, Box<dyn Error>> {
        self.repository.save_records_with_uuid(&self.records)
    }

    /// Returns a slice of all records in memory
    pub fn get_all_records(&self) -> &[LakeSpeciesRecord] {
        &self.records
    }

    /// Returns a single record by its 0-based index in the sequential list
    pub fn get_record_by_index(&self, index: usize) -> Option<&LakeSpeciesRecord> {
        self.records.get(index)
    }

    /// Creates and stores a new record in the in-memory sequential list
    pub fn create_record(&mut self, record: LakeSpeciesRecord) {
        self.records.push(record);
    }

    /// Replaces/updates an existing record at the given 0-based index
    pub fn update_record(&mut self, index: usize, updated: LakeSpeciesRecord) -> Result<(), String> {
        if let Some(target) = self.records.get_mut(index) {
            *target = updated;
            Ok(())
        } else {
            Err(format!("Record index {} is out of bounds.", index + 1))
        }
    }

    /// Deletes a record at the given 0-based index and returns the removed record
    pub fn delete_record(&mut self, index: usize) -> Result<LakeSpeciesRecord, String> {
        if index < self.records.len() {
            Ok(self.records.remove(index))
        } else {
            Err(format!("Record index {} is out of bounds.", index + 1))
        }
    }

    /// Sorts the in-memory Vec<LakeSpeciesRecord> in place based on a single dataset column
    pub fn sort_records(&mut self, column: SortColumn, ascending: bool) {
        self.records.sort_by(|a, b| {
            let ord: Ordering = match column {
                SortColumn::Identification => a.identification.cmp(&b.identification),
                SortColumn::LakeIdentification => a.lake_identification.cmp(&b.lake_identification),
                SortColumn::Year => a.year.cmp(&b.year),
                SortColumn::SpeciesCode => a.species_code.cmp(&b.species_code),
                SortColumn::Species => a.species.cmp(&b.species),
                SortColumn::PercentAbundance => a.percent_abundance.total_cmp(&b.percent_abundance),
            };

            if ascending {
                ord
            } else {
                ord.reverse()
            }
        });
    }
}


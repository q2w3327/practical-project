//! Business Layer mananges record
//!
//! Holds the [`Vec<LakeSpeciesRecord>`] container in memory and coordinates
//! handlesCRUD operations, sorting algorithms, dataset reloading, and CSV persistence.

use crate::model::LakeSpeciesRecord;
use crate::persistence::CsvRepository;
use std::cmp::Ordering;
use std::error::Error;

/// Identifies which dataset column to use when sorting records in memory.
#[derive(Debug, Clone, Copy)]
pub enum SortColumn {
    /// Sort by number in `Identification`.
    Identification,
    /// Sort by letter in `Lake identification`.
    LakeIdentification,
    /// Sort year  in `Year`.
    Year,
    /// Sort by letter  in `Species code`.
    SpeciesCode,
    /// Sort by letter in scientific `Species` name.
    Species,
    /// Sort by number by `Percent abundance`.
    PercentAbundance,
}

/// Service handles record collection and business rules.
pub struct LakeSpeciesService {
    /// Sequential data structure holding loaded records in memory.
    records: Vec<LakeSpeciesRecord>,
    /// Persistence repository used for reading and writing CSV files.
    repository: CsvRepository,
    /// Maximum number of initial records to load on startup or reload.
    record_limit: usize,
}

impl LakeSpeciesService {
    /// Creates a new [`LakeSpeciesService`]
    ///
    /// # Arguments
    /// * `repository` - The instance used for File-I/O.
    /// * `record_limit` - Maximum number of records allowed
    pub fn new(repository: CsvRepository, record_limit: usize) -> Self {
        Self {
            records: Vec::new(),
            repository,
            record_limit,
        }
    }

    /// Initializes the collection on startup 
    ///
    /// # Errors
    /// Returns an error if the dataset file is missing
    pub fn initialize_records(&mut self) -> Result<(), Box<dyn Error>> {
        self.reload_records()
    }

    /// Reloads  records from disk
    ///
    /// # Errors
    /// Returns an error if the file cannot be opened
    /// remains intact if loading fails.
    pub fn reload_records(&mut self) -> Result<(), Box<dyn Error>> {
        let fresh_records = self.repository.fetch_records(self.record_limit)?;
        self.records.clear();
        self.records = fresh_records;
        Ok(())
    }

    /// Persists all current in-memory records to a new CSV file named with a generated UUID.
    ///
    /// # Returns
    /// The `<uuid>.csv` filename on success.
    ///
    /// # Errors
    /// Returns an error if the output file cannot be created or written
    pub fn persist_to_new_csv(&self) -> Result<String, Box<dyn Error>> {
        self.repository.save_records_with_uuid(&self.records)
    }

    /// Returns a read-only slice of all [`LakeSpeciesRecord`] items.
    pub fn get_all_records(&self) -> &[LakeSpeciesRecord] {
        &self.records
    }

    /// Retrieves a reference to a single record by its 0-based `index`, or `None` if out of bounds.
    pub fn get_record_by_index(&self, index: usize) -> Option<&LakeSpeciesRecord> {
        self.records.get(index)
    }

    /// Appends a new [`LakeSpeciesRecord`] to the end of the in-memory sequential list.
    pub fn create_record(&mut self, record: LakeSpeciesRecord) {
        self.records.push(record);
    }

    /// Replaces an existing record at the given 0-based `index` with `updated`.
    ///
    /// # Errors
    /// Returns an `Err` message if `index` exceeds the current number of records.
    pub fn update_record(&mut self, index: usize, updated: LakeSpeciesRecord) -> Result<(), String> {
        if let Some(target) = self.records.get_mut(index) {
            *target = updated;
            Ok(())
        } else {
            Err(format!("Record index {} is out of bounds.", index + 1))
        }
    }

    /// Removes and returns the record at the given 0-based `index`.
    ///
    /// # Errors
    /// Returns an `Err` message if `index` is out of bounds.
    pub fn delete_record(&mut self, index: usize) -> Result<LakeSpeciesRecord, String> {
        if index < self.records.len() {
            Ok(self.records.remove(index))
        } else {
            Err(format!("Record index {} is out of bounds.", index + 1))
        }
    }

    /// Sorts the in-memory [`Vec<LakeSpeciesRecord>`] in place by the chosen [`SortColumn`].
    ///
    /// # Arguments
    /// * `column` - The dataset column to sort by.
    /// * `ascending` - `true` for ascending order (`A -> Z`, `0 -> 100`), `false` for descending.
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




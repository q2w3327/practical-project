//! Presentation Layer handles all console output, menus, and user prompts.
//!
//! Ensures Jacob Johnston remains visible across every view, interactive operation, and error state.

use crate::business::{LakeSpeciesService, SortColumn};
use crate::model::LakeSpeciesRecord;
use std::error::Error;
use std::io::{self, Write};

/// Persistent author name displayed across all console screens.
const AUTHOR_NAME: &str = "Jacob Johnston";

/// Stateless console view providing interactive user prompts and formatted output.
pub struct ConsoleView;

impl ConsoleView {
    /// Prints a persistent banner so Jacob Johnston always remains visible.
    pub fn show_author_banner() {
        println!("\n==========================================================================================");
        println!(" Program Author: {}", AUTHOR_NAME);
        println!("==========================================================================================");
    }

    /// Shows an initial startup status message before loading the dataset.
    pub fn show_startup_message(file_path: &str, max_records: usize) {
        println!(
            "Attempting to open '{}' and load up to {} records...",
            file_path, max_records
        );
    }

    /// Runs the main loop until the user exits.
    pub fn run_interactive_loop(service: &mut LakeSpeciesService, dataset_path: &str) {
        loop {
            Self::show_author_banner();
            println!(
                "Main Menu | Records currently in memory: {}",
                service.get_all_records().len()
            );
            println!("  1. Display records (single record or multiple records)");
            println!("  2. Create a new record in memory");
            println!("  3. Edit an existing record in memory");
            println!("  4. Delete a record from memory");
            println!("  5. Reload data from dataset (replaces in-memory data)");
            println!("  6. Persist in-memory data to a new CSV file (UUID filename)");
            println!("  7. Sort records in memory by column");
            println!("  8. Exit");

            let choice = Self::prompt_line("\nSelect an option (1-8): ");

            match choice.as_str() {
                "1" => Self::handle_display_menu(service),
                "2" => Self::handle_create_record(service),
                "3" => Self::handle_edit_record(service),
                "4" => Self::handle_delete_record(service),
                "5" => Self::handle_reload(service, dataset_path),
                "6" => Self::handle_persist(service),
                "7" => Self::handle_sort_records(service),
                "8" => {
                    println!("\nExiting program. Goodbye! (Author: {})", AUTHOR_NAME);
                    break;
                }
                _ => println!("\n[Invalid Option] Please enter a number between 1 and 8."),
            }
        }
    }

    /// Tells the user to view either a single record or a range of the stored records.
    fn handle_display_menu(service: &LakeSpeciesService) {
        let total = service.get_all_records().len();
        if total == 0 {
            println!("\n[Info] No records are currently stored in memory.");
            return;
        }

        Self::show_author_banner();
        println!("Display Options (1 to {} available):", total);
        println!("  1. Display a single record (by record number)");
        println!("  2. Display multiple records (all or a custom range)");
        let sub_choice = Self::prompt_line("Choose display mode (1-2): ");

        match sub_choice.as_str() {
            "1" => {
                if let Some(idx) = Self::prompt_record_index(total) {
                    if let Some(record) = service.get_record_by_index(idx) {
                        Self::show_author_banner();
                        println!("--- Single Record View [#{}] ---", idx + 1);
                        println!("{}", record);
                        println!("------------------------------------------------------------------------------------------");
                        println!("Program Author: {}", AUTHOR_NAME);
                    }
                }
            }
            "2" => {
                let range_choice =
                    Self::prompt_line("Enter 'all' for all records, or 'start-end' (e.g., 1-10): ");
                if range_choice.eq_ignore_ascii_case("all") {
                    Self::display_record_slice(service.get_all_records(), 0);
                } else if let Some((start_str, end_str)) = range_choice.split_once('-') {
                    match (start_str.trim().parse::<usize>(), end_str.trim().parse::<usize>()) {
                        (Ok(start), Ok(end)) if start >= 1 && start <= end && end <= total => {
                            let slice = &service.get_all_records()[(start - 1)..end];
                            Self::display_record_slice(slice, start - 1);
                        }
                        _ => println!(
                            "[Error] Invalid range. Please enter values between 1 and {}.",
                            total
                        ),
                    }
                } else {
                    println!("[Error] Invalid format. Use 'all' or 'start-end' (e.g., 1-15).");
                }
            }
            _ => println!("[Invalid Option] Returning to main menu."),
        }
    }

    /// Formats [`LakeSpeciesRecord`] items.
    pub fn display_record_slice(records: &[LakeSpeciesRecord], start_offset: usize) {
        Self::show_author_banner();
        println!("--- Displaying {} Lake Species Records ---", records.len());
        for (i, record) in records.iter().enumerate() {
            println!("[{:>3}] {}", start_offset + i + 1, record);
        }
        println!("------------------------------------------------------------------------------------------");
        println!(
            "Displayed: {} records | Program Author: {}",
            records.len(),
            AUTHOR_NAME
        );
        println!("==========================================================================================");
    }

    /// Collects field inputs and appends a new record.
    fn handle_create_record(service: &mut LakeSpeciesService) {
        Self::show_author_banner();
        println!("--- Create New Lake Species Record ---");

        let identification: u32 = match Self::prompt_line("Identification (integer): ").parse() {
            Ok(v) => v,
            Err(_) => {
                println!("[Error] Invalid Identification number. Aborting creation.");
                return;
            }
        };

        let lake_identification = Self::prompt_line("Lake identification (e.g., OCF19): ");
        if lake_identification.is_empty() {
            println!("[Error] Lake identification cannot be empty.");
            return;
        }

        let year: u16 = match Self::prompt_line("Year (e.g., 2012): ").parse() {
            Ok(v) => v,
            Err(_) => {
                println!("[Error] Invalid Year. Aborting creation.");
                return;
            }
        };

        let species_code = Self::prompt_line("Species code (e.g., AchMin): ");
        let species = Self::prompt_line("Species (e.g., Achnanthes minutissima): ");

        let percent_abundance: f64 = match Self::prompt_line("Percent abundance (e.g., 12.5): ").parse()
        {
            Ok(v) => v,
            Err(_) => {
                println!("[Error] Invalid Percent abundance. Aborting creation.");
                return;
            }
        };

        let new_record = LakeSpeciesRecord {
            identification,
            lake_identification,
            year,
            species_code,
            species,
            percent_abundance,
        };

        service.create_record(new_record.clone());
        println!(
            "\n[Success] Added new record at position #{}:",
            service.get_all_records().len()
        );
        println!("  {}", new_record);
    }


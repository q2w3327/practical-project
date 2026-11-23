use crate::business::LakeSpeciesService;
use crate::model::LakeSpeciesRecord;
use std::error::Error;
use std::io::{self, Write};

const AUTHOR_NAME: &str = "Jacob Johnston";

pub struct ConsoleView;

impl ConsoleView {
    /// Persistent banner so Jacob Johnston always remains visible
    pub fn show_author_banner() {
        println!("\n==========================================================================================");
        println!(" Program Author: {}", AUTHOR_NAME);
        println!("==========================================================================================");
    }

    pub fn show_startup_message(file_path: &str, max_records: usize) {
        println!(
            "Attempting to open '{}' and load up to {} records...",
            file_path, max_records
        );
    }

    /// Interactive user menu loop in the Presentation Layer
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
            println!("  7. Exit");

            let choice = Self::prompt_line("\nSelect an option (1-7): ");

            match choice.as_str() {
                "1" => Self::handle_display_menu(service),
                "2" => Self::handle_create_record(service),
                "3" => Self::handle_edit_record(service),
                "4" => Self::handle_delete_record(service),
                "5" => Self::handle_reload(service, dataset_path),
                "6" => Self::handle_persist(service),
                "7" => {
                    println!("\nExiting program. Goodbye! (Author: {})", AUTHOR_NAME);
                    break;
                }
                _ => println!("\n[Invalid Option] Please enter a number between 1 and 7."),
            }
        }
    }

    /// Sub-menu to display either a single record or multiple records
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

    /// Helper to print any slice of records while preserving their 1-based list numbers
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

    /// Prompts the user for fields to create a new record in memory
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
    /// Prompts the user to select an existing record and edit its fields
    fn handle_edit_record(service: &mut LakeSpeciesService) {
        let total = service.get_all_records().len();
        if total == 0 {
            println!("\n[Info] No records in memory to edit.");
            return;
        }

        Self::show_author_banner();
        println!("--- Edit Existing Record (1 to {}) ---", total);
        let idx = match Self::prompt_record_index(total) {
            Some(i) => i,
            None => return,
        };

        let current = service.get_record_by_index(idx).unwrap().clone();
        println!("\nCurrent Record [#{}]:\n  {}", idx + 1, current);
        println!("(Press Enter to keep the current value for any field)\n");

        let id_input = Self::prompt_line(&format!("Identification [{}]: ", current.identification));
        let identification = if id_input.is_empty() {
            current.identification
        } else {
            match id_input.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("[Error] Invalid number. Edit cancelled.");
                    return;
                }
            }
        };

        let lake_input = Self::prompt_line(&format!(
            "Lake identification [{}]: ",
            current.lake_identification
        ));
        let lake_identification = if lake_input.is_empty() {
            current.lake_identification
        } else {
            lake_input
        };

        let year_input = Self::prompt_line(&format!("Year [{}]: ", current.year));
        let year = if year_input.is_empty() {
            current.year
        } else {
            match year_input.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("[Error] Invalid year. Edit cancelled.");
                    return;
                }
            }
        };

        let code_input = Self::prompt_line(&format!("Species code [{}]: ", current.species_code));
        let species_code = if code_input.is_empty() {
            current.species_code
        } else {
            code_input
        };

        let species_input = Self::prompt_line(&format!("Species [{}]: ", current.species));
        let species = if species_input.is_empty() {
            current.species
        } else {
            species_input
        };

        let abund_input = Self::prompt_line(&format!(
            "Percent abundance [{}]: ",
            current.percent_abundance
        ));
        let percent_abundance = if abund_input.is_empty() {
            current.percent_abundance
        } else {
            match abund_input.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("[Error] Invalid percent abundance. Edit cancelled.");
                    return;
                }
            }
        };

        let updated = LakeSpeciesRecord {
            identification,
            lake_identification,
            year,
            species_code,
            species,
            percent_abundance,
        };

        match service.update_record(idx, updated.clone()) {
            Ok(()) => {
                println!("\n[Success] Record #{} updated to:", idx + 1);
                println!("  {}", updated);
            }
            Err(e) => println!("[Error] {}", e),
        }
    }

    /// Prompts the user to select and delete a record from memory
    fn handle_delete_record(service: &mut LakeSpeciesService) {
        let total = service.get_all_records().len();
        if total == 0 {
            println!("\n[Info] No records in memory to delete.");
            return;
        }

        Self::show_author_banner();
        println!("--- Delete Record (1 to {}) ---", total);
        let idx = match Self::prompt_record_index(total) {
            Some(i) => i,
            None => return,
        };

        if let Some(target) = service.get_record_by_index(idx) {
            println!("\nSelected Record [#{}]:\n  {}", idx + 1, target);
            let confirm = Self::prompt_line("Are you sure you want to delete this record? (y/n): ");
            if confirm.eq_ignore_ascii_case("y") {
                match service.delete_record(idx) {
                    Ok(removed) => println!("\n[Success] Deleted record: {}", removed),
                    Err(e) => println!("[Error] {}", e),
                }
            } else {
                println!("\n[Cancelled] Record was not deleted.");
            }
        }
    }

    /// Reloads up to 100 records from the source CSV, replacing memory
    fn handle_reload(service: &mut LakeSpeciesService, dataset_path: &str) {
        println!("\nReloading dataset from '{}'...", dataset_path);
        match service.reload_records() {
            Ok(()) => {
                println!(
                    "[Success] In-memory data replaced with {} freshly loaded records.",
                    service.get_all_records().len()
                );
            }
            Err(err) => {
                Self::display_error(dataset_path, &err);
                println!("Existing in-memory data was preserved.");
            }
        }
    }

    /// Saves the current in-memory records to a new CSV named with a UUID
    fn handle_persist(service: &LakeSpeciesService) {
        println!(
            "\nPersisting {} records to a new UUID CSV file...",
            service.get_all_records().len()
        );
        match service.persist_to_new_csv() {
            Ok(filename) => {
                println!(
                    "[Success] Saved {} records to '{}'.",
                    service.get_all_records().len(),
                    filename
                );
            }
            Err(err) => {
                eprintln!("[Persistence Error] Failed to write output CSV file: {}", err);
            }
        }
    }

    /// Helper to read a 1-based record number from the user and return a 0-based index
    fn prompt_record_index(total: usize) -> Option<usize> {
        let input = Self::prompt_line(&format!("Enter record number (1-{}): ", total));
        match input.parse::<usize>() {
            Ok(num) if num >= 1 && num <= total => Some(num - 1),
            _ => {
                println!("[Error] Invalid record number. Must be between 1 and {}.", total);
                None
            }
        }
    }

    /// Helper to display a prompt and read a trimmed line from stdin
    fn prompt_line(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        input.trim().to_string()
    }

    /// Formats and outputs startup/file exceptions cleanly to the user
    pub fn display_error(file_path: &str, err: &Box<dyn Error>) {
        if let Some(io_err) = err.downcast_ref::<io::Error>() {
            match io_err.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("\n[File Error] The file '{}' was not found.", file_path);
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("\n[File Error] Permission denied when opening '{}'.", file_path);
                }
                _ => {
                    eprintln!("\n[I/O Error] Failed to read '{}': {}", file_path, io_err);
                }
            }
        } else {
            eprintln!("\n[Data Error] {}", err);
        }
        println!("------------------------------------------------------------------------------------------");
        println!("Program Author: {}", AUTHOR_NAME);
        println!("==========================================================================================");
    }
}

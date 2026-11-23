    /// Prompts the user to choose a record
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

    /// Prompts the user to select and confirm deletion of a record from memory.
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

    /// Triggers a full reload from the CSV file, replacing the records.
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

    /// Saves the current records out to a new CSV file.
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

    /// Prompts the user to choose a column and sort direction, then sorts records.
    fn handle_sort_records(service: &mut LakeSpeciesService) {
        if service.get_all_records().is_empty() {
            println!("\n[Info] No records in memory to sort.");
            return;
        }

        Self::show_author_banner();
        println!("--- Sort Records by Column ---");
        println!("  1. Identification");
        println!("  2. Lake identification");
        println!("  3. Year");
        println!("  4. Species code");
        println!("  5. Species");
        println!("  6. Percent abundance");

        let col_choice = Self::prompt_line("\nSelect column to sort by (1-6): ");
        let (column, col_name) = match col_choice.as_str() {
            "1" => (SortColumn::Identification, "Identification"),
            "2" => (SortColumn::LakeIdentification, "Lake identification"),
            "3" => (SortColumn::Year, "Year"),
            "4" => (SortColumn::SpeciesCode, "Species code"),
            "5" => (SortColumn::Species, "Species"),
            "6" => (SortColumn::PercentAbundance, "Percent abundance"),
            _ => {
                println!("[Error] Invalid column selection. Sort cancelled.");
                return;
            }
        };

        let order_choice = Self::prompt_line("Sort order — (A)scending or (D)escending? [A/D]: ");
        let (ascending, order_label) = if order_choice.eq_ignore_ascii_case("d") {
            (false, "Descending")
        } else if order_choice.eq_ignore_ascii_case("a") || order_choice.is_empty() {
            (true, "Ascending")
        } else {
            println!("[Error] Invalid sort order. Use 'A' or 'D'. Sort cancelled.");
            return;
        };

        service.sort_records(column, ascending);
        println!(
            "\n[Success] Sorted {} records by '{}' ({}).",
            service.get_all_records().len(),
            col_name,
            order_label
        );
        Self::display_record_slice(service.get_all_records(), 0);
    }

    /// Prompts for a 1-based record number and converts it to a validated 0-based index.
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

    /// Displays `prompt`, flushes stdout, and reads a trimmed line from standard input.
    fn prompt_line(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        input.trim().to_string()
    }

    /// Displays user-friendly error messages while preserving the author footer.
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


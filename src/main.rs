//! Lake Species Dataset Manager
//!
//! **Author:** Jacob Johnston
//!
//! A layered Rust console application organized into four distinct modules:
//! - [`model`]: Domain entity object (`LakeSpeciesRecord`).
//! - [`persistence`]: CSV File-I/O and UUID outputn (`CsvRepository`).
//! - [`business`]: In-memory sequential list, CRUD operations, and sorting (`LakeSpeciesService`).
//! - [`presentation`]: Interactive console user interface (`ConsoleView`).

mod business;
mod model;
mod persistence;
mod presentation;

use business::LakeSpeciesService;
use persistence::CsvRepository;
use presentation::ConsoleView;

/// Default path to the CSV dataset.
const DATASET_PATH: &str = "dataset.csv";

/// Maximum number of records loaded on startup and reload.
const INITIAL_RECORD_LIMIT: usize = 100;

/// Application entry point wiring the Persistence, Business, and Presentation layers.
fn main() {
    ConsoleView::show_author_banner();
    ConsoleView::show_startup_message(DATASET_PATH, INITIAL_RECORD_LIMIT);

    let repository = CsvRepository::new(DATASET_PATH);
    let mut service = LakeSpeciesService::new(repository, INITIAL_RECORD_LIMIT);

    match service.initialize_records() {
        Ok(()) => {
            ConsoleView::display_record_slice(service.get_all_records(), 0);
            ConsoleView::run_interactive_loop(&mut service, DATASET_PATH);
        }
        Err(err) => {
            ConsoleView::display_error(DATASET_PATH, &err);
            std::process::exit(1);
        }
    }
}

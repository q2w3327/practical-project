mod business;
mod model;
mod persistence;
mod presentation;

use business::LakeSpeciesService;
use persistence::CsvRepository;
use presentation::ConsoleView;

const DATASET_PATH: &str = "dataset.csv";
const INITIAL_RECORD_LIMIT: usize = 100;

fn main() {
    // 1. Presentation Layer: Show persistent author header on startup
    ConsoleView::show_author_banner();
    ConsoleView::show_startup_message(DATASET_PATH, INITIAL_RECORD_LIMIT);

    // 2. Wire Persistence Layer -> Business Layer
    let repository = CsvRepository::new(DATASET_PATH);
    let mut service = LakeSpeciesService::new(repository, INITIAL_RECORD_LIMIT);

    // 3. Load up to 100 records (or all if < 100) into Business Layer memory on startup
    match service.initialize_records() {
        Ok(()) => {
            // 4. Display initial records, then launch full CRUD interactive menu
            ConsoleView::display_record_slice(service.get_all_records(), 0);
            ConsoleView::run_interactive_loop(&mut service, DATASET_PATH);
        }
        Err(err) => {
            // 4. Handle missing/unreadable file errors gracefully
            ConsoleView::display_error(DATASET_PATH, &err);
            std::process::exit(1);
        }
    }
}
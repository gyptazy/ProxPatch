use log::LevelFilter;
use systemd_journal_logger::JournalLog;

pub fn init(debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    match JournalLog::new() {
        Ok(journal) => {
            if journal.install().is_err() {
                env_logger::init();
            }
        }
        Err(_) => {
            env_logger::init();
        }
    }

    if debug {
        log::set_max_level(LevelFilter::Debug);
    } else {
        log::set_max_level(LevelFilter::Info);
    }

    Ok(())
}
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;

pub struct DualLogger {
    log_file: Option<Mutex<File>>,
}

impl DualLogger {
    pub fn new(log_path: Option<&str>) -> io::Result<Self> {
        let log_file = if let Some(path) = log_path {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(Path::new(path))?;
            Some(Mutex::new(file))
        } else {
            None
        };

        Ok(DualLogger { log_file })
    }

    pub fn log(&self, message: &str) -> io::Result<()> {
        use chrono::Local;
        // Get current date and time
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S");
        let formatted_message = format!("[{timestamp}] {message}");

        // Write to terminal
        eprintln!("{formatted_message}");

        // Write to file if configured
        if let Some(file) = &self.log_file {
            let mut file = file.lock().unwrap();
            writeln!(file, "{formatted_message}")?;
            file.flush()?;
        }

        Ok(())
    }
}

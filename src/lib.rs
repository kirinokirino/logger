use log::{LevelFilter, Metadata, Record, SetLoggerError};

pub struct BareLogger {
    logging_level: LevelFilter,
}

impl BareLogger {
    #[must_use = "To enable logging, call .init()."]
    pub fn new(level: LevelFilter) -> Self {
        Self {
            logging_level: level,
        }
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.logging_level);
        log::set_boxed_logger(Box::new(self))?;
        Ok(())
    }
}

impl log::Log for BareLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.logging_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let target = if record.target().is_empty() {
                "".to_string()
            } else {
                format!("({})", record.target())
            };
            println!("[{:<5}]: {} \t{}", record.level(), record.args(), target);
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use log::*;

    #[test]
	fn log() {
		BareLogger::new(LevelFilter::Debug).init();

		trace!("trace");
		debug!("debug");
		info!("info");
		warn!("warn");
		error!("error");
		panic!("Kappa");
    }

}

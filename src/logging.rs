use cortex_m_semihosting::hprintln;

pub struct SemihostingLogger;

static MY_LOGGER: SemihostingLogger = SemihostingLogger;

pub fn init() {
    log::set_logger(&MY_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}

impl log::Log for SemihostingLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        hprintln!(
            "<{}>\t{}:  {}",
            record.level(),
            record.target(),
            record.args()
        )
        .unwrap();
    }
    fn flush(&self) {}
}

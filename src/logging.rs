use rtt_target::{rprintln, set_print_channel, UpChannel};

use panic_rtt_target as _;

struct RTTLogger;
static RTT_LOGGER: RTTLogger = RTTLogger;

pub fn init(channel: UpChannel) {
    set_print_channel(channel);

    log::set_logger(&RTT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    rprintln!("\r\n");
}

impl log::Log for RTTLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        rprintln!(
            "<{}> {}\r",
            record.level(),
            // record.target(),
            record.args()
        )
    }
    fn flush(&self) {}
}

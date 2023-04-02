use log::{Level, LevelFilter, Metadata, Record};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("⟨{}⟩ {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn init(level: LevelFilter) {
    log::set_logger(&SimpleLogger)
        .map(|()| log::set_max_level(level))
        .unwrap();
}

macro_rules! make {
    ($f:ident, $en:ident) => {
        #[allow(unused)]
        pub fn $f() {
            init(LevelFilter::$en);
        }
    };
}

make!(info, Info);
make!(debug, Debug);
make!(error, Error);

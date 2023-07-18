use sysinfo::SystemExt;
use tracing::{info, Level};

mod logger;
mod recv;

fn main() {
    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_line_number(true)
                .with_level(true)
                .with_target(true),
        )
        .with_max_level(Level::INFO)
        .try_init()
        .unwrap();
    _ = std::fs::create_dir_all("./msglog");
    let sys = sysinfo::System::new_all();
    for id in 1..sys.cpus().len() {
        std::thread::spawn(move || {
            #[cfg(target_os = "linux")]
            let _ = monoio::RuntimeBuilder::<monoio::IoUringDriver>::new()
                .with_entries(16384)
                .enable_timer()
                .build()
                .unwrap()
                .block_on(recv::start(id));
            #[cfg(target_os = "macos")]
            let _ = monoio::RuntimeBuilder::<monoio::LegacyDriver>::new()
                .enable_timer()
                .build()
                .unwrap()
                .block_on(recv::start(id));
        });
    }
    info!("msglogger starting...");
    #[cfg(target_os = "linux")]
    let _ = monoio::RuntimeBuilder::<monoio::IoUringDriver>::new()
        .with_entries(16384)
        .enable_timer()
        .build()
        .unwrap()
        .block_on(recv::start(0));
    #[cfg(target_os = "macos")]
    let _ = monoio::RuntimeBuilder::<monoio::LegacyDriver>::new()
        .enable_timer()
        .build()
        .unwrap()
        .block_on(recv::start(0));
}

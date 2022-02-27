use snek_rs::tui::Tui;

use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

use std::panic;

fn main() {
    panic::set_hook(Box::new(|e| {
        tracing::info!("panicked:\n{:?}", e);
    }));
    let current_dir = std::env::current_dir().unwrap();
    let log_file = "tui-snek.log";
    let file_appender = tracing_appender::rolling::never(current_dir, log_file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let format = tracing_subscriber::fmt::format().without_time().pretty();
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .event_format(format)
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();
    let mut tui = Tui::new();
    tui.run()
}

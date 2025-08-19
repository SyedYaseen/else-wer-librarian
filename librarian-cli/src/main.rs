mod cli;
mod file_scan;
mod models;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use tracing_appender::rolling;
use tracing_subscriber::{
    EnvFilter, Layer,
    fmt::{self, time::UtcTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::cli::run;
use file_scan::file_scan;

#[tokio::main]
async fn main() -> Result<()> {
    // color_eyre::install()?;
    // let mut terminal = ratatui::init();
    // let result = run(terminal);
    // ratatui::restore();
    // result

    // tracing
    // let file_appender = rolling::daily("logs", "else-wer.log");
    // let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);

    // let console_filter =
    //     EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // let file_filter = EnvFilter::new("info");

    // let stdout_layer = fmt::layer()
    //     .with_target(false)
    //     .with_file(true)
    //     .with_thread_ids(true)
    //     .with_timer(UtcTime::rfc_3339())
    //     .with_line_number(true)
    //     .compact()
    //     .with_filter(console_filter);

    // let file_layer = fmt::layer()
    //     .json()
    //     .with_target(true)
    //     .with_file(true)
    //     .with_line_number(true)
    //     .with_thread_ids(true)
    //     .with_writer(non_blocking_file)
    //     .with_filter(file_filter);

    // tracing_subscriber::registry()
    //     .with(stdout_layer)
    //     .with(file_layer)
    //     .init();

    // std::mem::forget(_guard);

    // tracing end

    let _ = file_scan("/media/yaseen/Data/AudioBooks".into()).await;
    // let _ = file_scan("/media/yaseen/Data/AudioBooks/JamesSACorey".into());
    Ok(())
}

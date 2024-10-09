//! Rust logging demo.

use log::{debug, error, info};
use logger;

/// Logs a greeting.
fn main() {
    logger::init_once(
        logger::Config::default()
            .with_tag("rust")
            .with_min_level(log::Level::Trace),
    );
    debug!("Starting program.");
    info!("Things are going fine.");
    error!("Something went wrong!");
}

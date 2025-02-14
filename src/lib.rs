mod rune_arc;
mod regulation;
mod util;

use broadsword::{dll, logging};

#[dll::entrypoint]
pub fn entry(_: usize) -> bool {
    logging::init("auto-arc.log");
    rune_arc::hook();
    regulation::hook();
    log::info!("AutoArc loading complete.");
    true
}
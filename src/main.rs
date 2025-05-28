mod app;
mod types;
mod ui;

use anyhow::{Context, Result};
use app::App;

const TICK_DURATION: u64 = 10_000_000; // 10ms

fn main() -> Result<()> {
    let state = App::new(TICK_DURATION);
    let terminal = ratatui::init();
    let app_result = ui::run(terminal, state).context("app loop failed");
    ratatui::restore();
    app_result
}

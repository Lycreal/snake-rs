use crate::app::App;

use anyhow::{Context, Result};
use ratatui::crossterm::event::{self, KeyCode, KeyModifiers};
use ratatui::{Terminal, backend::Backend};

pub fn run<B: Backend>(mut terminal: Terminal<B>, mut state: App) -> Result<()> {
    loop {
        if event::poll(state.next_tick()).context("event poll failed")? {
            if let Ok(event::Event::Key(key)) = event::read() {
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    // handle quit event
                    break;
                }
                // handle key event
                state
                    .handle_key_event(key)
                    .context("handle key event failed")?;
            }
        } else {
            // handle tick event
            state
                .handle_tick_event(&mut terminal)
                .context("handle tick event failed")?;
        }
    }
    Ok(())
}

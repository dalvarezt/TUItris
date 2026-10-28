pub mod event;
pub mod game;
use std::io::{self, stdout, Stdout};
use std::panic;
use std::time::Duration;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::cursor::{Hide, Show};
use ratatui::backend::CrosstermBackend;
use ratatui::{Terminal, restore};
use event::{Event, EventHandler};

pub type TuiTerminal = Terminal<CrosstermBackend<Stdout>>;

pub fn init_terminal() -> io::Result<TuiTerminal> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn restore_terminal() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen, Show)?;
    disable_raw_mode()
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));
}

fn main() -> io::Result<()> {
    install_panic_hook();
    let mut terminal = init_terminal()?;
    let events = EventHandler::new(Duration::from_millis(33)); // ~30 FPS polling loop
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let text = "TUItris Phase 1 Initialized. Press 'q' or 'Esc' to exit.";
            let line = ratatui::text::Line::from(text);
            frame.render_widget(line, area);
        })?;
        match events.next().map_err(|e| io::Error::other(e))? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                _ => {}
            },
            Event::Tick => {}
            Event::Resize(_, _) => {}
        }
    }
    restore_terminal()?;
    Ok(())
}

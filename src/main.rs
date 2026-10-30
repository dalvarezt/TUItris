pub mod app;
pub mod event;
pub mod game;
pub mod ui;

use std::io::{self, stdout, Stdout};
use std::panic;
use std::time::Duration;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::cursor::{Hide, Show};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal; 
use event::{Event, EventHandler};
use app::App;

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
    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let events = EventHandler::new(tick_rate); // ~30 FPS polling loop
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if app.should_quit() {
            break;
        }

        match events.next().map_err(io::Error::other)? {
            Event::Key(key) => app.handle_key(key),
            Event::Tick => app.tick(tick_rate),
            Event::Resize(_, _) => {}
        }
    }
    restore_terminal()?;
    Ok(())
}

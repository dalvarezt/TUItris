pub mod game_over;
pub mod leaderboard;
pub mod pause_modal;
pub mod playfield;
pub mod sidebar;
pub mod start_menu;

use ratatui::layout::{Alignment, Constraint, Direction, Flex, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::app::{App, AppState};

pub const MIN_WIDTH: u16 = 60;
pub const MIN_HEIGHT: u16 = 26;

/// Helper to compute a horirzontally and vertically centered Rect inside `r`
pub fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .flex(Flex::Center)
        .constraints([Constraint::Length(height)])
        .split(r);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .flex(Flex::Center)
        .constraints([Constraint::Length(width)])
        .split(vertical[0]);
    
    horizontal[0]
}

/// Root render entry point. Checks terminal dimensions and dispatches views
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Adaptive ermnal Resize Warning
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        let warning_text = Text::from(vec![
            Line::from("Terminal size too small!"),
            Line::from(format!("Current: {}x{}", area.width, area.height)),
            Line::from(format!("Required: at least {}x{}", MIN_WIDTH, MIN_HEIGHT)),
        ]);
        let warning = Paragraph::new(warning_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title(" Warning "),
            );
        let warning_area = centered_rect(40, 70, area);
        frame.render_widget(warning, warning_area);
        return;
    }
    
    match app.state() {
        AppState::StartMenu => start_menu::render(frame, area), 
        AppState::Playing => {
            render_gameplay(frame, area, app);
        }
        AppState::Paused => {
            render_gameplay(frame, area, app);
            pause_modal::render(frame, area);
        }
        AppState::GameOver => {
            render_gameplay(frame, area, app);
            game_over::render(frame, area, app);
        }
        AppState::Leaderboard => leaderboard::render(frame, area, app),
    }
}

fn render_gameplay(frame: &mut Frame, area: Rect, app: &App) {
    let container = centered_rect(48, 22, area);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Length(26)])
        .split(container);
    playfield::render(frame, chunks[0], app.engine());
    sidebar::render(frame, chunks[1], app.engine());
}

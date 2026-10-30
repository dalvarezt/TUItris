use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use super::centered_rect;

pub fn render(frame: &mut Frame, area: Rect) {
    let modal_area = centered_rect(52, 18, area);
    let banner = vec![
          Line::from(Span::styled(" _____ _   _ _____ _        _     ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
          Line::from(Span::styled("|_   _| | | |_   _| |      (_)    ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
          Line::from(Span::styled("  | | | | | | | | | |_ _ __ _ ___ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
          Line::from(Span::styled("  | | | | | | | | | __| '__| / __|", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
          Line::from(Span::styled("  | | | |_| |_| |_| |_| |  | \\__ \\", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
          Line::from(Span::styled("  \\_/  \\___/|_____|\\__|_|  |_|___/", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
          Line::from(""),
          Line::from(Span::styled("Terminal Tetris in Rust", Style::default().fg(Color::DarkGray))),
          Line::from(""),
          Line::from(vec![
              Span::styled("[Enter] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
              Span::raw("Start New Game"),
          ]),
          Line::from(vec![
              Span::styled("[H]     ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
              Span::raw("View High Scores"),
          ]),
          Line::from(vec![
              Span::styled("[Q]     ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
              Span::raw("Quit Game"),
          ]),
     ];

    let menu = Paragraph::new(banner)
         .alignment(Alignment::Center)
         .block(
             Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Cyan)),
         );
    frame.render_widget(menu, modal_area);
}

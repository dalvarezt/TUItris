use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

use super::centered_rect;

pub fn render(frame: &mut Frame, area: Rect) {
    let popup_area = centered_rect(29, 7, area);

    let text = vec![
        Line::from(""),
        Line::from(Span::styled("GAME PAUSED", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("[P] Resume | [Q] Quit"),
    ];

    let modal = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Yellow)),
        );
    
    // Clear underneathh to obscure the board
    frame.render_widget(Clear, popup_area);
    frame.render_widget(modal, popup_area);
}
    
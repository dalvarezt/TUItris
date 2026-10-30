use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

use super::centered_rect;
use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let popup_area = centered_rect(36, 12, area);
    let engine = app.engine();

    let mut lines = vec![
        Line::from(Span::styled("GAME OVER", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(format!("Score: {}", engine.score())),
        Line::from(format!("Level: {} | Lines {}", engine.level(), engine.lines() )),
        Line::from(""),
    ];

    if app.is_high_score() {
        lines.push(Line::from(Span::styled("NEW HIGH SCORE!", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))));
        let initials_display = format!("{:<3}", app.initials_input());
        lines.push(
            Line::from(vec![
                Span::raw("Enter initials: ["),
                Span::styled(initials_display, Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
                Span::raw("]"),
            ])
        );
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("[Enter] Save | [Esc] Skip", Style::default().fg(Color::DarkGray))));
    } else {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("[Enter] Leaderboard | [R] Restart", Style::default().fg(Color::Cyan))));
    }

    let widget = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Red)),
        );
    
    frame.render_widget(Clear, popup_area);
    frame.render_widget(widget, popup_area);
}
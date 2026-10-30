use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::game::engine::Engine;
use crate::game::tetromino::Piece;

pub fn render(frame: &mut Frame, area: Rect, engine: &Engine) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Stats (score, level lines)
            Constraint::Length(6), // Next piece (4x4 box)
            Constraint::Length(11), // Controls and help
        ])
        .split(area);
    
    // Stats card
    let stats = vec![
        Line::from(vec![
            Span::styled("Score: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:>6}", engine.score()), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Level: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:>6}", engine.level()), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Lines: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:>6}", engine.lines()), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
    ];
    let stats_widget = Paragraph::new(stats).block(
        Block::default()
           .borders(Borders::ALL)
           .border_type(BorderType::Rounded)
           .title(" Stats "),
    );
    frame.render_widget(stats_widget, chunks[0]);

    // Next piece card
    let next_kind = engine.next_kind();
    let next_piece = Piece::new(next_kind);
    let next_color = next_kind.color();
    let blocks: Vec<(i32, i32)> = next_piece.blocks().collect();

    let mut preview_lines = Vec::with_capacity(4);
    for r in 0..next_piece.size {
        let mut spans = Vec::new();

        // visual centering inside 4x4 box
        let pad = (4 - next_piece.size) / 2;
        for _ in 0..pad {
            spans.push(Span::raw("  "));
        }
        for c in 0..next_piece.size {
            if blocks.contains(&(c as i32, r as i32)) {
                spans.push(Span::styled("🮆🮆", Style::default().fg(next_color)));
            } else {
                spans.push(Span::raw("  "));
            }
        }
        preview_lines.push(Line::from(spans));
    }
    let next_widget = Paragraph::new(preview_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Next Piece "),
    );
    frame.render_widget(next_widget, chunks[1]);

    // Controls and help card
    let controls = vec![
        Line::from(vec![Span::styled("← / →", Style::default().fg(Color::Cyan)), Span::raw(" : Move")]),
        Line::from(vec![Span::styled("↑    ", Style::default().fg(Color::Cyan)), Span::raw(" : Rotate")]),
        Line::from(vec![Span::styled("↓    ", Style::default().fg(Color::Cyan)), Span::raw(" : Soft Drop")]),
        Line::from(vec![Span::styled("Space", Style::default().fg(Color::Cyan)), Span::raw(" : Hard Drop")]),
        Line::from(vec![Span::styled("P    ", Style::default().fg(Color::Cyan)), Span::raw(" : Pause")]),
        Line::from(vec![Span::styled("Q    ", Style::default().fg(Color::Cyan)), Span::raw(" : Forfeit")]),
    ];
    let controls_widget = Paragraph::new(controls).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Controls "),
    );
    frame.render_widget(controls_widget, chunks[2]);
}

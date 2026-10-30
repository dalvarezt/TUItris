use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::game::board::{BUFFER_ROWS, HEIGHT, WIDTH};
use crate::game::engine::Engine;

pub fn render(frame: &mut Frame, area: Rect, engine: &Engine) {
    let board = engine.board();
    let active = engine.active();
    let ghost_y = engine.ghost_y();

    // Map active piece cells
    let active_blocks: Vec<(i32, i32)> = active
        .piece
        .blocks()
        .map(|(dx, dy)| (active.x + dx, active.y +dy))
        .collect();

    // Map ghost piece cells
    let ghost_blocks: Vec<(i32, i32)> = active
        .piece
        .blocks()
        .map(|(dx, dy)| (active.x + dx, ghost_y + dy))
        .collect();

    let mut lines = Vec::with_capacity(HEIGHT - BUFFER_ROWS);

    for y in BUFFER_ROWS..HEIGHT {
        let mut spans = Vec::with_capacity(WIDTH);
        for x in 0..WIDTH {
            let px = x as i32;
            let py = y as i32;

            if let Some(color) = board.cell(x,y) {
                // Locked stack block
                spans.push(Span::styled("🮆🮆", Style::default().fg(color)));
            } else if active_blocks.contains(&(px, py)) {
                // Active falling piece
                spans.push(Span::styled("🮆🮆", Style::default().fg(active.piece.kind.color())));
            } else if ghost_blocks.contains(&(px, py)) {
                // Ghost piece (dimmed color)
                spans.push(Span::styled("⣿⣿", Style::default().fg(Color::DarkGray)));
            } else {
                // empty cell
                spans.push(Span::styled(" .", Style::default().fg(Color::Rgb(40, 40, 40))));
            }
        }
        lines.push(Line::from(spans));
    }

    let widget = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Board "),
    );

    frame.render_widget(widget, area);
}

use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Row, Table};
use ratatui::Frame;

use super::centered_rect;
use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
  let board_area = centered_rect(48, 18, area);

  let header_cells = ["Rank", "Name", "Score", "Level", "Date"]
      .into_iter()
      .map(|h| Cell::from(h).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
  let header = Row::new(header_cells).height(1).bottom_margin(1);

  let scores = app.high_scores();
  let rows: Vec<Row> = scores
      .iter()
      .enumerate()
      .map(|(i, entry)| {
          let rank = format!("#{}", i + 1);
          let cells = vec![
              Cell::from(rank).style(Style::default().fg(Color::Yellow)),
              Cell::from(entry.initials.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
              Cell::from(entry.score.to_string()),
              Cell::from(entry.level.to_string()),
              Cell::from(entry.date.clone()).style(Style::default().fg(Color::DarkGray)),
          ];
          Row::new(cells).height(1)
      })
      .collect();

  let widths = [
      Constraint::Length(6),
      Constraint::Length(6),
      Constraint::Length(10),
      Constraint::Length(8),
      Constraint::Length(12),
  ];

  let table = Table::new(rows, widths)
      .header(header)
      .block(
          Block::default()
              .borders(Borders::ALL)
              .border_type(BorderType::Double)
              .title(" High Scores ")
              .title_alignment(Alignment::Center),
      );

  frame.render_widget(table, board_area);
}



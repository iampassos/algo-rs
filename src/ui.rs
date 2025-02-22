use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, List, Padding, Widget},
};

use crate::app;

impl Widget for &app::App {
    fn render(self, rect: Rect, buffer: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(rect);

        let graph_layout = centered_rect(80, 55, rect);

        let title = Line::from(" Rust Algorithm Visualizer ")
            .bold()
            .green()
            .centered();

        let instructions = Line::from(vec![
            " Quit ".red().bold(),
            "<Q> ".blue().bold(),
            "Pause/Resume ".red().bold(),
            "<P> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let complete_style = if self.completed {
            Style::new().green()
        } else {
            Style::new().white()
        };

        let bars: Vec<Bar> = self
            .array
            .iter()
            .map(|n| {
                Bar::default()
                    .value_style(complete_style.reversed())
                    .value(u64::from(*n))
            })
            .collect();

        let clone = block.clone();

        let barchart = BarChart::default()
            .block(block.padding(Padding {
                left: 1,
                right: 0,
                top: 0,
                bottom: 0,
            }))
            .bar_width(2)
            .bar_gap(1)
            .bar_style(complete_style)
            .value_style(complete_style)
            .label_style(complete_style)
            .data(BarGroup::default().bars(bars.as_slice()));

        let title = Line::from(" Overview ").centered();

        let inner_block = Block::bordered().title(title).border_set(border::THICK);
        let inner = clone.inner(layout[0]);

        let layout_inner = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(inner);

        let status = if self.completed {
            "Completed"
        } else if self.paused {
            "Paused"
        } else {
            "Running"
        };

        let overview = List::new(Line::from(vec![
            format!("Algorithm: {}", self.algorithm).into(),
            format!("Total Numbers: {}", self.array.len()).into(),
            format!("Iterations: {}", self.iterations).into(),
            format!("Time Elapsed: {:.2}s", self.elapsed).into(),
            format!("Status: {}", status).into(),
        ]));

        barchart.render(graph_layout, buffer);

        overview.block(inner_block).render(layout_inner[0], buffer);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(layout[1])[1]
}

use std::io::Result;

use crossterm::event;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, List, Widget},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        data: std::array::from_fn(|i| (i + 1) as u32),
        algorithm: Some(Algorithm {
            name: "Bubble Sort".to_string(),
            iterations: 0,
            time_elapsed: 0,
        }),
    };

    let result = app.run(&mut terminal);

    ratatui::restore();

    result
}

struct Algorithm {
    name: String,
    iterations: u32,
    time_elapsed: u32,
}

struct App {
    exit: bool,
    data: [u32; 100],
    algorithm: Option<Algorithm>,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            if self.exit {
                return Ok(());
            }

            match event::read()? {
                event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => (),
            }

            _ = terminal.draw(|f| self.draw(f));
        }
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<()> {
        if key_event.kind == event::KeyEventKind::Press {
            match key_event.code {
                event::KeyCode::Char('q') => self.exit = true,
                _ => (),
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, rect: Rect, buffer: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(rect);

        let title = Line::from(" Rust Algorithm Visualizer ")
            .bold()
            .blue()
            .centered();

        let instructions = Line::from(vec![" Quit ".yellow().bold(), "<Q> ".bold().blue()]);

        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let bars: Vec<Bar> = self
            .data
            .iter()
            .map(|n| {
                Bar::default()
                    .value_style(Style::default().white().bg(Color::White))
                    .value(u64::from(*n))
            })
            .collect();

        let clone = block.clone();

        let barchart = BarChart::default()
            .block(block)
            .bar_width(1)
            .bar_gap(0)
            .bar_style(Style::new().white())
            .value_style(Style::new().white())
            .data(BarGroup::default().bars(bars.as_slice()));

        let title = Line::from(" Overview ").centered();

        let inner_block = Block::bordered().title(title).border_set(border::THICK);
        let inner = clone.inner(layout[0]);

        let layout_inner = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(inner);

        let layout_inner_inner = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(layout_inner[0]);

        let mut overview = List::new(Line::from(vec![
            "Algorithm: None".into(),
            format!("Total Numbers: {}", self.data.len()).into(),
            "Iterations: 0".into(),
            "Time Elapsed: 0s".into(),
        ]));

        barchart.render(layout[0], buffer);

        if let Some(algorithm) = &self.algorithm {
            overview = List::new(Line::from(vec![
                format!("Algorithm: {}", algorithm.name).into(),
                format!("Total Numbers: {}", self.data.len()).into(),
                format!("Iterations: {}", algorithm.iterations).into(),
                format!("Time Elapsed: {}s", algorithm.time_elapsed).into(),
            ]))
        }

        overview
            .block(inner_block)
            .render(layout_inner_inner[0], buffer);
    }
}

use std::{io::Result, sync::mpsc, thread, time::Instant};

use crossterm::event;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, List, Padding},
    Frame,
};

use crate::{
    algorithms::Algorithm,
    array::Array,
    state::{SharedState, State, Status},
};

pub enum Event {
    Input(event::KeyEvent),
}

pub struct App {
    pub exit: bool,
    state: SharedState,
}

impl App {
    pub fn new(array: Vec<u32>) -> Self {
        Self {
            exit: false,
            state: SharedState::new(State {
                array,
                iterations: 0,
                last_swapped: 0,
                status: Status::Paused,
                start: Instant::now(),
                end: Instant::now(),
                algorithm: String::from("None"),
            }),
        }
    }

    pub fn run(&self, algorithm: Box<dyn Algorithm + Send>, tx: mpsc::Sender<Event>) -> Result<()> {
        let algorithm_state1 = self.state.clone();
        let algorithm_state2 = self.state.clone();

        thread::spawn(move || algorithm.sort(algorithm_state1, Array::new(algorithm_state2)));

        thread::spawn(move || loop {
            match event::read().unwrap() {
                event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
                _ => {}
            }
        });

        Ok(())
    }

    pub fn handle_input(&mut self, key_event: event::KeyEvent) {
        if key_event.kind == event::KeyEventKind::Press {
            match key_event.code {
                event::KeyCode::Char('q') => self.exit = true,
                _ => {}
            }
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(frame.area());

        let graph_layout = centered_rect(80, 55, frame.area());

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

        let status = self.state.get_status();
        let start = self.state.get_start();
        let algorithm = self.state.get_algorithm();
        let array = self.state.get().array.clone();
        let last = self.state.get_last();
        let iterations = self.state.get().iterations;

        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let completed_style = Style::new().green();

        let complete_style = if let Status::Completed = status {
            completed_style
        } else {
            Style::new().white()
        };

        let bars: Vec<Bar> = array
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let bar = Bar::default();

                if i == last as usize {
                    bar.style(completed_style)
                        .value_style(completed_style.reversed())
                        .value(u64::from(*n))
                } else {
                    bar.value_style(complete_style.reversed())
                        .value(u64::from(*n))
                }
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

        let status_text = match status {
            Status::Completed => "Completed",
            Status::Paused => "Paused",
            Status::Running => "Running",
        };

        let overview = List::new(Line::from(vec![
            format!("Algorithm: {}", algorithm).into(),
            format!("Total Numbers: {}", array.len()).into(),
            format!("Iterations: {}", iterations).into(),
            format!(
                "Time Elapsed: {:.2}s",
                if let Status::Paused | Status::Completed = status {
                    (self.state.get_end() - start).as_secs_f32()
                } else {
                    start.elapsed().as_secs_f32()
                }
            )
            .into(),
            format!("Status: {}", status_text).into(),
        ]));

        frame.render_widget(barchart, graph_layout);
        frame.render_widget(overview.block(inner_block), layout_inner[0]);
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

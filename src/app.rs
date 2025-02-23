use std::thread;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, List, Padding, Widget},
    Frame,
};

use crate::{
    algorithms::Algorithm,
    array::Array,
    state::{SharedState, State},
};

pub struct App {
    state: SharedState,
}

impl App {
    pub fn new(array: Vec<u32>) -> Self {
        Self {
            state: SharedState::new(State {
                array,
                iterations: 0,
                last_swapped: 0,
                completed: false,
            }),
        }
    }

    pub fn run(&self, algorithm: Box<dyn Algorithm + Send>) {
        let algorithm_state1 = self.state.clone();
        let algorithm_state2 = self.state.clone();

        thread::spawn(move || algorithm.sort(algorithm_state1, Array::new(algorithm_state2)));
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

        let completed = self.state.get_completed();
        let array = self.state.get().array.clone();
        let iterations = self.state.get().iterations;

        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let complete_style = if completed {
            Style::new().green()
        } else {
            Style::new().white()
        };

        let bars: Vec<Bar> = array
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

        let status = if completed { "Completed" } else { "Running" };

        let overview = List::new(Line::from(vec![
            //format!("Algorithm: {}", self.algorithm).into(),
            format!("Total Numbers: {}", array.len()).into(),
            format!("Iterations: {}", iterations).into(),
            //format!("Time Elapsed: {:.2}s", self.elapsed).into(),
            format!("Status: {}", status).into(),
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

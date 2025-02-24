use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

use crossterm::{event, style::Color};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Borders, List},
    Frame,
};

use crate::{
    algorithms::{bubble_sort::BubbleSort, Algorithm},
    array::Array,
    state::{SharedState, State, Status},
};

pub enum Event {
    Input(event::KeyEvent),
}

pub struct App {
    pub exit: bool,
    state: SharedState,
    algorithm_handle: Option<JoinHandle<()>>,
}

impl App {
    pub fn new(array: Vec<u32>) -> Self {
        Self {
            exit: false,
            state: SharedState::new(State::new(array)),
            algorithm_handle: None,
        }
    }

    pub fn init(tx: mpsc::Sender<Event>) -> Self {
        let array = App::generate_array();
        let state = App::new(array);

        thread::spawn(move || loop {
            match event::read().unwrap() {
                event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
                _ => {}
            }
        });

        state
    }

    pub fn handle_input(&mut self, key_event: event::KeyEvent) {
        if key_event.kind == event::KeyEventKind::Press {
            match key_event.code {
                event::KeyCode::Char('q') => self.exit = true,
                event::KeyCode::Char('p') => {
                    if let Some(ref handle) = self.algorithm_handle {
                        let status = self.state.get_status();

                        if let Status::Paused = status {
                            self.state.set_status(Status::Running);
                            handle.thread().unpark();
                        };

                        if let Status::Running = status {
                            self.state.set_status(Status::Paused);
                        };
                    }
                }
                event::KeyCode::Char('1') => self.start_algorithm(Box::new(BubbleSort)),
                _ => {}
            }
        }
    }

    pub fn interrupt_algorithm(&mut self) {
        if let Some(_) = self.algorithm_handle {
            self.state.set_status(Status::Interrupted);
            self.state = SharedState::new(State::new(App::generate_array()));
            self.algorithm_handle = None;
        }
    }

    pub fn start_algorithm(&mut self, algorithm: Box<dyn Algorithm + Send>) {
        self.interrupt_algorithm();

        let algorithm_state1 = self.state.clone();
        let algorithm_state2 = self.state.clone();

        let algorithm_handle =
            thread::spawn(move || algorithm.sort(algorithm_state1, Array::new(algorithm_state2)));

        self.algorithm_handle = Some(algorithm_handle);
    }

    pub fn generate_array() -> Vec<u32> {
        let mut arr: Vec<u32> = [0; 150].to_vec();

        for i in 0..150 {
            arr[i] = 150 as u32 - i as u32;
        }

        arr
    }

    pub fn draw(&self, frame: &mut Frame) {
        let State {
            array,
            last_swapped,
            comparison,
            checked,
            iterations,
            status,
            start,
            end,
            algorithm,
        } = self.state.get().clone();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(frame.area());

        let graph_layout = centered_rect(79, 55, frame.area());

        let block = Block::new()
            .title(Line::bold(format!(" {algorithm} ").into()).centered())
            .border_set(border::THICK)
            .borders(Borders::BOTTOM);

        let completed_style = Style::new().green();
        let comparison_style = Style::new().red();
        let normal_style = Style::new().white();

        let style = match status {
            Status::Completed => completed_style,
            _ => normal_style,
        };

        let bars: Vec<Bar> = array
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let bar = Bar::default();

                let completed_bar = Bar::default()
                    .style(completed_style)
                    .value_style(completed_style.on_green())
                    .value(u64::from(*n));

                if let Status::Completed = status {
                    completed_bar
                } else if i == last_swapped as usize || checked.contains(&u32::try_from(i).unwrap())
                {
                    completed_bar
                } else if comparison.contains(&u32::try_from(i).unwrap()) {
                    bar.style(comparison_style)
                        .value_style(comparison_style.on_red())
                        .value(u64::from(*n))
                } else {
                    bar.style(style)
                        .value_style(style.on_white())
                        .value(u64::from(*n))
                }
            })
            .collect();

        let barchart = BarChart::default()
            .block(block)
            .bar_width(1)
            .bar_gap(0)
            .bar_style(style)
            .value_style(style)
            .label_style(style)
            .data(BarGroup::default().bars(bars.as_slice()));

        let layout_inner = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(1),
                Constraint::Percentage(15),
                Constraint::Fill(1),
                Constraint::Percentage(10),
                Constraint::Percentage(1),
            ])
            .split(layout[0]);

        let overview_block = Block::new()
            .title(Line::raw(" Overview ").centered())
            .borders(Borders::TOP);
        let overview_rect = overview_block.inner(layout_inner[1]);

        let help_block = Block::new()
            .title(Line::raw(" Help ").centered())
            .borders(Borders::TOP);
        let help_rect = help_block.inner(layout_inner[3]);

        let status_text = match status {
            Status::Completed => "Completed",
            Status::Paused => "Paused",
            Status::Running => "Running",
            Status::Interrupted => "Interrupted",
            Status::Checking => "Checking",
            Status::Failed => "Failed",
        };

        let status_color = match status {
            Status::Completed => Color::Green,
            Status::Running => Color::White,
            Status::Paused => Color::Yellow,
            Status::Interrupted => Color::Red,
            Status::Checking => Color::Yellow,
            Status::Failed => Color::Red,
        };

        let overview = List::new(
            Line::from(vec![
                format!("Algorithm: {}", algorithm).into(),
                format!("Total Numbers: {}", array.len()).into(),
                format!("Iterations: {}", iterations).into(),
                format!(
                    "Time Elapsed: {:.2}s",
                    if let Status::Paused | Status::Failed | Status::Checking | Status::Completed =
                        status
                    {
                        (end - start).as_secs_f32()
                    } else {
                        start.elapsed().as_secs_f32()
                    }
                )
                .into(),
                format!("Status: {}", status_text).fg(status_color).into(),
            ])
            .centered(),
        );

        let help = List::new(
            Line::from(vec![
                "Quit: <Q>".into(),
                "Pause/Resume: <P>".into(),
                "Next: <L>".into(),
                "Previous: <H>".into(),
            ])
            .centered(),
        );

        frame.render_widget(barchart, graph_layout);
        frame.render_widget(overview.block(overview_block), overview_rect);
        frame.render_widget(help.block(help_block), help_rect);
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

use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

use rand::prelude::*;

use crossterm::{event, style::Color};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, BorderType, Borders, List, Paragraph, Wrap},
    Frame,
};

use crate::{
    algorithms::{
        bubble_sort::BubbleSort, insertion_sort::InsertionSort, merge_sort::MergeSort,
        quick_sort::QuickSort, selection_sort::SelectionSort, Algorithm,
    },
    array::Array,
    state::{SharedState, State, Status},
};

pub enum Event {
    Input(event::KeyEvent),
}

pub struct App {
    pub exit: bool,
    pub state: SharedState,
    algorithm_handle: Option<JoinHandle<()>>,
    algorithm_index: i8,
}

impl App {
    pub fn new(array: Vec<u32>) -> Self {
        Self {
            exit: false,
            state: SharedState::new(State::new(array)),
            algorithm_handle: None,
            algorithm_index: 0,
        }
    }

    pub fn init(tx: mpsc::Sender<Event>) -> Self {
        let array = App::generate_array();
        let mut state = App::new(array);

        thread::spawn(move || loop {
            match event::read().unwrap() {
                event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
                _ => {}
            }
        });

        state.handle_algorithms(0);

        state
    }

    pub fn handle_input(&mut self, key_event: event::KeyEvent) {
        if key_event.kind == event::KeyEventKind::Press {
            match key_event.code {
                event::KeyCode::Char('k') => {
                    self.state.increment_speed();
                }
                event::KeyCode::Char('j') => {
                    self.state.decrement_speed();
                }
                event::KeyCode::Char('h') => self.handle_algorithms(-1),
                event::KeyCode::Char('l') => self.handle_algorithms(1),
                event::KeyCode::Char('r') => self.handle_algorithms(0),
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
                _ => {}
            }
        }
    }

    pub fn handle_algorithms(&mut self, increment: i8) {
        let algorithm_count = 5;

        if increment <= 0 && self.algorithm_index == 0 {
            self.algorithm_index = algorithm_count;
        } else {
            self.algorithm_index += increment;
        }

        match self.algorithm_index % algorithm_count {
            1 => self.start_algorithm(Box::new(SelectionSort)),
            2 => self.start_algorithm(Box::new(InsertionSort)),
            3 => self.start_algorithm(Box::new(MergeSort)),
            4 => self.start_algorithm(Box::new(QuickSort)),
            _ => self.start_algorithm(Box::new(BubbleSort)),
        }
    }

    pub fn interrupt_algorithm(&mut self) {
        if let Some(_) = self.algorithm_handle {
            self.state.set_status(Status::Interrupted);
            self.state = SharedState::new(State::new(App::generate_array()));
            self.algorithm_handle = None;
        }
    }

    pub fn start_algorithm(&mut self, algorithm: Box<dyn Algorithm + Send + Sync>) {
        self.interrupt_algorithm();

        let algorithm_state1 = self.state.clone();
        let algorithm_state2 = self.state.clone();

        let algorithm_handle =
            thread::spawn(move || algorithm.sort(algorithm_state1, Array::new(algorithm_state2)));

        self.algorithm_handle = Some(algorithm_handle);
    }

    pub fn generate_array() -> Vec<u32> {
        let mut rng = rand::rng();
        let mut nums: Vec<u32> = (1..151).collect();
        nums.shuffle(&mut rng);
        nums
    }

    pub fn draw(&self, frame: &mut Frame) {
        let State {
            array,
            last_swapped,
            comparison,
            checked,
            array_accesses,
            comparisons,
            status,
            algorithm,
            log,
            speed,
        } = self.state.get().clone();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(frame.area());

        let graph_layout = centered_rect(80, 55, frame.area());

        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);

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
                let completed_bar = Bar::default()
                    .style(completed_style)
                    .value_style(completed_style.on_green())
                    .value(u64::from(*n));

                let comparison_bar = Bar::default()
                    .style(comparison_style)
                    .value_style(comparison_style.on_red())
                    .value(u64::from(*n));

                if let Status::Completed = status {
                    completed_bar
                } else if i == last_swapped as usize || checked.contains(&u32::try_from(i).unwrap())
                {
                    completed_bar
                } else if comparison.contains(&u32::try_from(i).unwrap()) {
                    comparison_bar
                } else if let Status::Failed = status {
                    comparison_bar
                } else {
                    Bar::default()
                        .style(style)
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
                Constraint::Percentage(73),
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

        let title_block = Block::new()
            .title(Line::raw(" algorithm-tui ").bold().centered())
            .borders(Borders::TOP);
        let title_rect = overview_block.inner(layout_inner[2]);

        let (status_text, status_color) = match status {
            Status::Completed => ("Completed", Color::Green),
            Status::Paused => ("Paused", Color::Yellow),
            Status::Running => ("Running", Color::White),
            Status::Interrupted => ("Interrupted", Color::Red),
            Status::Checking => ("Checking", Color::Yellow),
            Status::Failed => ("Failed", Color::Red),
        };

        let overview = List::new(Line::from(vec![
            format!("Algorithm: {}", algorithm).fg(Color::Green).into(),
            format!("Total Numbers: {}", array.len()).into(),
            format!("Array Accesses: {}", array_accesses).into(),
            format!("Comparisons: {}", comparisons).into(),
            format!("Speed: {}%", speed).into(),
            format!("Status: {}", status_text).fg(status_color).into(),
        ]));

        let help = List::new(Line::from(vec![
            "Quit: <Q>".into(),
            "Pause/Resume: <P>".into(),
            "Reset: <R>".into(),
            "Next: <L>".into(),
            "Previous: <H>".into(),
            "Increase Speed: <K>".into(),
            "Decrease Speed: <J>".into(),
        ]));

        frame.render_widget(barchart, graph_layout);
        frame.render_widget(overview.block(overview_block), overview_rect);
        frame.render_widget(help.block(help_block), help_rect);
        frame.render_widget(title_block, title_rect);

        if let Some(text) = log {
            frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: true }), layout[2]);
        }
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

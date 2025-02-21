use std::{
    io::Result,
    sync::mpsc,
    thread::{self},
    time::{self},
};

use crossterm::event;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, List, Padding, Widget},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new();

    let (tx, rx) = mpsc::channel();

    let clone1 = tx.clone();
    thread::spawn(move || handle_input_events(clone1));

    let clone2 = tx.clone();
    thread::spawn(move || handle_algorithm(clone2));

    let result = app.run(&mut terminal, rx);

    ratatui::restore();

    result
}

fn reset_array() -> [u32; 100] {
    let mut arr: [u32; 100] = [0; 100];

    for i in 0..100 {
        arr[i] = 100 - i as u32;
    }

    arr
}

struct ProgressEvent {
    name: String,
    array: [u32; 100],
    iterations: u32,
    start: time::Instant,
}

enum Event {
    Input(event::KeyEvent),
    Progress(ProgressEvent),
}

fn handle_input_events(tx: mpsc::Sender<Event>) -> Result<()> {
    loop {
        match event::read()? {
            event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => (),
        }
    }
}

fn handle_algorithm(tx: mpsc::Sender<Event>) -> Result<()> {
    let start = time::Instant::now();

    let mut array = reset_array();
    let len = array.len();

    let mut iterations = 0;

    for i in 0..len - 1 {
        let mut swap = false;

        for j in 0..len - i - 1 {
            if array[j] > array[j + 1] {
                swap = true;
                array.swap(j, j + 1);

                tx.send(Event::Progress(ProgressEvent {
                    name: String::from("Bubble Sort"),
                    array,
                    iterations,
                    start,
                }))
                .unwrap()
            }

            iterations += 1;
        }

        if !swap {
            break;
        }
    }
    Ok(())
}

struct App {
    exit: bool,
    array: [u32; 100],
    algorithm: String,
    iterations: u32,
    elapsed: f32,
}

impl App {
    fn new() -> Self {
        Self {
            exit: false,
            array: reset_array(),
            algorithm: String::from("None"),
            iterations: 0,
            elapsed: 0_f32,
        }
    }
    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> Result<()> {
        while !self.exit {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_input(key_event)?,
                Event::Progress(event) => self.handle_progress(event)?,
            }

            terminal.draw(|f| self.draw(f))?;
        }

        Ok(())
    }

    fn handle_progress(&mut self, event: ProgressEvent) -> Result<()> {
        self.algorithm = event.name;
        self.array = event.array;
        self.iterations = event.iterations;
        self.elapsed = event.start.elapsed().as_secs_f32();

        Ok(())
    }

    fn handle_input(&mut self, key_event: event::KeyEvent) -> Result<()> {
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
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(rect);

        let graph_layout = centered_rect(54, 55, rect);

        let title = Line::from(" Rust Algorithm Visualizer ")
            .bold()
            .green()
            .centered();

        let instructions = Line::from(vec![" Quit ".red().bold(), "<Q> ".blue().bold()]);

        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let bars: Vec<Bar> = self
            .array
            .iter()
            .map(|n| {
                Bar::default()
                    .value_style(Style::default().white().bg(Color::White))
                    .value(u64::from(*n))
            })
            .collect();

        let clone = block.clone();

        let barchart = BarChart::default()
            .block(block.padding(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            }))
            .bar_width(1)
            .bar_gap(0)
            .bar_style(Style::new().white())
            .value_style(Style::new().white())
            .data(BarGroup::default().bars(bars.as_slice()));

        let title = Line::from(" Overview ").centered();

        let inner_block = Block::bordered().title(title).border_set(border::THICK);
        let inner = clone.inner(layout[0]);

        let layout_inner = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(inner);

        let overview = List::new(Line::from(vec![
            format!("Algorithm: {}", self.algorithm).into(),
            format!("Total Numbers: {}", self.array.len()).into(),
            format!("Iterations: {}", self.iterations).into(),
            format!("Time Elapsed: {:.2}s", self.elapsed).into(),
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

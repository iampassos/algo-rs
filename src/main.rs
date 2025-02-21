use std::io::Result;

use crossterm::event;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App { exit: false };

    let result = app.run(&mut terminal);

    ratatui::restore();

    result
}

struct App {
    exit: bool,
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
        let title = Line::from(" Rust Algorithm Visualizer ")
            .bold()
            .blue()
            .centered();
        let instructions = Line::from(vec![" Quit ".yellow().bold(), "<Q> ".bold().blue()]);

        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .render(rect, buffer);
    }
}

use std::{io::Result, sync::mpsc};

use crossterm::event;
use ratatui::{DefaultTerminal, Frame};

pub type ArraySize = [u32; 50];

use crate::events::{AlgorithmEvent, Event};

// move to utils later
pub fn reset_array() -> ArraySize {
    let mut arr: ArraySize = [0; 50];

    for i in 0..50 {
        arr[i] = 50 as u32 - i as u32;
    }

    arr
}
pub struct App {
    pub exit: bool,
    pub array: ArraySize,
    pub algorithm: String,
    pub iterations: u32,
    pub elapsed: f32,
    pub paused: bool,
    pub completed: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            array: reset_array(),
            algorithm: String::from("None"),
            iterations: 0,
            elapsed: 0_f32,
            paused: false,
            completed: false,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rx: mpsc::Receiver<Event>,
        tx: mpsc::Sender<AlgorithmEvent>,
    ) -> Result<()> {
        while !self.exit {
            match rx.recv().unwrap() {
                Event::Input(key_event) => {
                    if key_event.kind == event::KeyEventKind::Press {
                        match key_event.code {
                            event::KeyCode::Char('q') => self.exit = true,
                            event::KeyCode::Char('p') => {
                                self.paused = if self.paused { false } else { true };
                                if !self.completed {
                                    tx.send(AlgorithmEvent::Pause).unwrap()
                                }
                            }
                            _ => (),
                        }
                    }
                }
                Event::Progress(event) => {
                    self.algorithm = event.name;
                    self.array = event.array;
                    self.iterations = event.iterations;
                    self.elapsed = event.start.elapsed().as_secs_f32();
                }
                Event::Complete(status) => self.completed = status,
            }

            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

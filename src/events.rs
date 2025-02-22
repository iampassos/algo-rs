use std::{
    io::Result,
    sync::mpsc,
    thread::{self},
    time::{self},
};

use crossterm::event;

use crate::app;

pub struct ProgressEvent {
    pub name: String,
    pub array: app::ArraySize,
    pub iterations: u32,
    pub start: time::Instant,
}

pub enum Event {
    Input(event::KeyEvent),
    Progress(ProgressEvent),
    Complete(bool),
}

pub enum AlgorithmEvent {
    Pause,
}

pub fn handle_input_events(tx: mpsc::Sender<Event>) -> Result<()> {
    loop {
        match event::read()? {
            event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

pub fn handle_algorithm(tx: mpsc::Sender<Event>, rx: mpsc::Receiver<AlgorithmEvent>) -> Result<()> {
    let start = time::Instant::now();

    let mut array = app::reset_array();
    let len = array.len();

    let mut iterations = 0;

    for i in 0..len - 1 {
        let mut swap = false;

        for j in 0..len - i - 1 {
            if let Ok(event) = rx.try_recv() {
                match event {
                    AlgorithmEvent::Pause => {
                        rx.recv().unwrap();
                    }
                }
            };

            if array[j] > array[j + 1] {
                swap = true;
                array.swap(j, j + 1);

                tx.send(Event::Progress(ProgressEvent {
                    name: String::from("Bubble Sort"),
                    array,
                    iterations,
                    start,
                }))
                .unwrap();

                thread::sleep(time::Duration::from_millis(10));
            }

            iterations += 1;
        }

        if !swap {
            break;
        }
    }

    tx.send(Event::Complete(true)).unwrap();

    Ok(())
}

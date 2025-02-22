use std::{
    io::Result,
    sync::mpsc,
    thread::{self},
};

pub mod app;
pub mod events;
pub mod ui;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let mut app = app::App::new();

    let (tx, rx) = mpsc::channel();

    let (alg_tx, alg_rx) = mpsc::channel();

    let clone1 = tx.clone();
    thread::spawn(move || events::handle_input_events(clone1));

    let clone2 = tx.clone();
    thread::spawn(move || events::handle_algorithm(clone2, alg_rx));

    let result = app.run(&mut terminal, rx, alg_tx);

    ratatui::restore();

    result
}

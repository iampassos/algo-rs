use std::{io::Result, sync::mpsc, thread, time};

pub mod algorithms;
pub mod app;
pub mod array;
pub mod state;

use app::App;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let (tx, rx) = mpsc::channel::<app::Event>();

    let mut app = App::init(tx.clone());

    while !app.exit {
        let event = rx.try_recv();

        if let Ok(event) = event {
            match event {
                app::Event::Input(key_event) => app.handle_input(key_event),
            }
        }

        terminal.draw(|frame| app.draw(frame)).unwrap();

        thread::sleep(time::Duration::from_millis(20));
    }

    ratatui::restore();

    Ok(())
}

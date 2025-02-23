use std::{io::Result, sync::mpsc, thread, time};

use algorithms::bubble_sort::BubbleSort;

pub mod algorithms;
pub mod app;
pub mod array;
pub mod state;

use app::App;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let (tx, rx) = mpsc::channel::<app::Event>();

    let mut app = App::new(generate_array());
    app.run(Box::new(BubbleSort), tx.clone()).unwrap();

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

pub fn generate_array() -> Vec<u32> {
    let mut arr: Vec<u32> = [0; 50].to_vec();

    for i in 0..50 {
        arr[i] = 50 as u32 - i as u32;
    }

    arr
}

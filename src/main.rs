use std::io::Result;

use algorithms::bubble_sort::BubbleSort;

pub mod algorithms;
pub mod app;
pub mod array;
pub mod state;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let app = app::App::new(generate_array());
    app.run(Box::new(BubbleSort));

    loop {
        terminal.draw(|frame| app.draw(frame)).unwrap();
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

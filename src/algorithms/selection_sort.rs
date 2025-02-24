use std::time;

use super::Algorithm;
use crate::{
    array::Array,
    state::{SharedState, Status},
};

pub struct SelectionSort;

impl Algorithm for SelectionSort {
    fn sort(&self, state: SharedState, array: Array) {
        state.set_algorithm("Selection Sort".to_string());
        state.set_status(Status::Running);
        state.set_start(time::Instant::now());

        let len = array.len();

        for i in 0..len - 1 {
            let mut min_index = i;

            for j in (i + 1)..len {
                match state.get_status() {
                    Status::Paused => state.park(),
                    Status::Interrupted => return,
                    _ => {}
                };

                if array.compare(min_index, j) {
                    min_index = j;
                }

                state.sleep(None);
            }

            if i != min_index {
                state.set_last(u32::try_from(i).unwrap());
                array.swap(i, min_index);
            }
        }

        state.check();
    }
}

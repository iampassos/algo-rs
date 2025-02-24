use std::time;

use super::Algorithm;
use crate::{
    array::Array,
    state::{SharedState, Status},
};

pub struct BubbleSort;

impl Algorithm for BubbleSort {
    fn sort(&self, state: SharedState, array: Array) {
        state.set_algorithm("Bubble Sort".to_string());
        state.set_status(Status::Running);
        state.set_start(time::Instant::now());

        let len = array.len();

        for i in 0..len - 1 {
            let mut swap = false;

            for j in 0..len - i - 1 {
                match state.get_status() {
                    Status::Paused => state.park(),
                    Status::Interrupted => return,
                    _ => {}
                };

                state.set_comparison([u32::try_from(j).unwrap(), u32::try_from(j + 1).unwrap()]);

                if array.get(j) > array.get(j + 1) {
                    swap = true;

                    state.set_last(u32::try_from(j + 1).unwrap());
                    array.swap(j, j + 1);
                }

                state.increment_iterations();
                state.sleep(None);
            }

            if !swap {
                break;
            }
        }

        state.check();
    }
}

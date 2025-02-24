use std::time;

use super::Algorithm;
use crate::{
    array::Array,
    state::{SharedState, Status},
};

pub struct InsertionSort;

impl Algorithm for InsertionSort {
    fn sort(&self, state: SharedState, array: Array) {
        state.set_algorithm("Insertion Sort".to_string());
        state.set_status(Status::Running);
        state.set_start(time::Instant::now());

        let len = array.len();

        for i in 1..len {
            match state.get_status() {
                Status::Paused => state.park(),
                Status::Interrupted => return,
                _ => {}
            };

            let mut left = i;

            while left > 0 && array.compare(left - 1, left) {
                match state.get_status() {
                    Status::Paused => state.park(),
                    Status::Interrupted => return,
                    _ => {}
                };

                state.set_last(u32::try_from(left).unwrap());
                array.swap(left - 1, left);
                left -= 1;

                state.sleep(None);
            }
        }

        state.check();
    }
}

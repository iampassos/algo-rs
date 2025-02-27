use super::Algorithm;
use crate::{
    array::Array,
    state::{SharedState, Status},
};

pub struct InsertionSort;

impl Algorithm for InsertionSort {
    fn sort(&self, state: SharedState, array: Array) {
        state.init_algorithm("Insertion Sort".to_string());

        let len = array.len();

        for i in 1..len {
            match state.get_status() {
                Status::Paused => state.park(),
                Status::Interrupted => return,
                _ => {}
            };

            let mut left = i;

            while left > 0 && array.is_greater(left - 1, left) {
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

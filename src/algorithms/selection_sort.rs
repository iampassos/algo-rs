use super::Algorithm;
use crate::{
    array::Array,
    state::{SharedState, Status},
};

pub struct SelectionSort;

impl Algorithm for SelectionSort {
    fn sort(&self, state: SharedState, array: Array) {
        state.init_algorithm("Selection Sort".to_string());

        let len = array.len();

        for i in 0..len - 1 {
            let mut min_index = i;

            for j in (i + 1)..len {
                match state.get_status() {
                    Status::Paused => state.park(),
                    Status::Interrupted => return,
                    _ => {}
                };

                if array.is_greater(min_index, j) {
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

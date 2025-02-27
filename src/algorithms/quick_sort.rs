use super::Algorithm;
use crate::{
    array::Array,
    state::{SharedState, Status},
};

pub struct QuickSort;

impl Algorithm for QuickSort {
    fn sort(&self, state: SharedState, array: Array) {
        state.set_speed(75);
        state.init_algorithm("Quick Sort".to_string());

        fn partition(state: &SharedState, array: &Array, start: usize, end: usize) -> usize {
            let (mut i, mut j) = (start, start as isize - 1);

            while end > i {
                match state.get_status() {
                    Status::Paused => state.park(),
                    _ => {}
                };

                if array.is_greater_equal(end, i) {
                    j += 1;

                    if i as isize != j {
                        state.set_last(j as u32);
                        array.swap(i, j as usize);
                    }
                }

                i += 1;
                state.sleep(None);
            }

            j += 1;

            state.set_last(j as u32);
            array.swap(j as usize, end);

            return j as usize;
        }

        fn quick_sort(state: &SharedState, array: &Array, start: usize, end: usize) {
            if end > start {
                let pivot = partition(state, array, start, end);

                if pivot > 0 {
                    quick_sort(state, array, start, pivot - 1);
                }

                quick_sort(state, array, pivot + 1, end);
            }
        }

        quick_sort(&state, &array, 0, array.len() - 1);

        state.check();
    }
}

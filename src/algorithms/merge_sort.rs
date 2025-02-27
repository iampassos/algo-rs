use super::Algorithm;
use crate::{
    array::Array,
    state::{SharedState, Status},
};

pub struct MergeSort;

impl Algorithm for MergeSort {
    fn sort(&self, state: SharedState, array: Array) {
        state.set_speed(75);
        state.init_algorithm("Merge Sort".to_string());

        fn merge_sort(state: &SharedState, array: &Array, start: u32, end: u32) -> () {
            if start >= end {
                return;
            }

            let mut mid = (start + end) / 2;

            merge_sort(state, &array, start, mid);
            merge_sort(state, &array, mid + 1, end);

            let (mut left, mut right) = (start, mid + 1);

            while left <= mid && right <= end {
                state.sleep(None);
                match state.get_status() {
                    Status::Paused => state.park(),
                    Status::Interrupted => return,
                    _ => {}
                };

                if array.is_greater(left as usize, right as usize) {
                    let mut idx: usize = right as usize;
                    let temp = array.get(idx);

                    while idx > left as usize {
                        array.set(idx, array.get(idx - 1));
                        idx -= 1;
                    }

                    state.set_last(left as u32);
                    array.set(left as usize, temp);

                    left += 1;
                    mid += 1;
                    right += 1;
                } else {
                    left += 1;
                }
            }
        }

        merge_sort(&state, &array, 0, array.len() as u32 - 1);

        state.check();
    }
}

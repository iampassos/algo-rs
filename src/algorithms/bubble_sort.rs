use super::Algorithm;
use crate::{array::Array, state::SharedState};

pub struct BubbleSort;

impl Algorithm for BubbleSort {
    fn sort(&self, state: SharedState, array: Array) {
        let len = array.len();

        for i in 0..len - 1 {
            let mut swap = false;

            for j in 0..len - i - 1 {
                if array.get(j) > array.get(j + 1) {
                    swap = true;

                    array.swap(j, j + 1);
                    state.set_last(u32::try_from(j).unwrap());
                }

                state.increment_iterations();
            }

            if !swap {
                break;
            }
        }

        state.set_completed(true);
    }
}

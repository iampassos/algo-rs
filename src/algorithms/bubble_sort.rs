use super::Algorithm;
use crate::{
    array::Array,
    state::{SharedState, Status},
};

pub struct BubbleSort;

impl Algorithm for BubbleSort {
    fn sort(&self, state: SharedState, array: Array) {
        state.init_algorithm("Bubble Sort".to_string());

        let len = array.len();

        for i in 0..len - 1 {
            let mut swap = false;

            for j in 0..len - i - 1 {
                match state.get_status() {
                    Status::Paused => state.park(),
                    Status::Interrupted => return,
                    _ => {}
                };

                if array.compare(j, j + 1) {
                    swap = true;
                    state.set_last(u32::try_from(j + 1).unwrap());
                    array.swap(j, j + 1);
                }

                state.sleep(None);
            }

            if !swap {
                break;
            }
        }

        state.check();
    }
}

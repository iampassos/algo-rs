use std::ops::Index;

use crate::state::SharedState;

#[derive(Clone)]
pub struct Array(SharedState);

impl Array {
    pub fn new(state: SharedState) -> Self {
        Self(state)
    }

    pub fn get(&self, index: usize) -> u32 {
        self.0.increment_accesses(1);
        *self.0.get().array.index(index)
    }

    pub fn get_all(&self) -> Vec<u32> {
        let state = self.0.get();
        state.array.to_vec()
    }

    pub fn set(&self, index: usize, value: u32) -> u32 {
        self.0.increment_accesses(1);
        let mut state = self.0.get();
        state.array[index] = value;
        value
    }

    pub fn len(&self) -> usize {
        self.0.get().array.len()
    }

    pub fn compare(&self, index1: usize, index2: usize) -> bool {
        self.0.increment_comparisons();
        self.0.set_comparison([
            u32::try_from(index1).unwrap(),
            u32::try_from(index2).unwrap(),
        ]);
        self.get(index1) > self.get(index2)
    }

    pub fn swap(&self, index1: usize, index2: usize) {
        self.0.get().array.swap(index1, index2);
        self.0.increment_accesses(4);
    }
}

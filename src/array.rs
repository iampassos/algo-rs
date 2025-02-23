use std::ops::Index;

use crate::state::SharedState;

#[derive(Clone)]
pub struct Array(SharedState);

impl Array {
    pub fn new(state: SharedState) -> Self {
        Self(state)
    }

    pub fn get(&self, index: usize) -> u32 {
        *self.0.get().array.index(index)
    }

    pub fn get_all(&self) -> Vec<u32> {
        let state = self.0.get();
        state.array.to_vec()
    }

    pub fn set(&self, index: usize, value: u32) -> u32 {
        let mut state = self.0.get();
        state.array[index] = value;
        value
    }

    pub fn len(&self) -> usize {
        self.0.get().array.len()
    }

    pub fn swap(&self, index1: usize, index2: usize) {
        self.0.get().array.swap(index1, index2);
    }
}

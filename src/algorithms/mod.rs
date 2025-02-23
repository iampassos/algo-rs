use crate::{array::Array, state::SharedState};

pub mod bubble_sort;

pub trait Algorithm {
    fn sort(&self, state: SharedState, array: Array);
}

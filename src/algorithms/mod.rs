use crate::{array::Array, state::SharedState};

pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod selection_sort;

pub trait Algorithm {
    fn sort(&self, state: SharedState, array: Array);
}

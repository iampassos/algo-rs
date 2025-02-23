use std::sync::{Arc, Mutex, MutexGuard};

pub struct State {
    pub array: Vec<u32>,
    pub last_swapped: u32,
    pub iterations: u32,
    pub completed: bool,
}

#[derive(Clone)]
pub struct SharedState(Arc<Mutex<State>>);

impl SharedState {
    pub fn new(state: State) -> Self {
        Self(Arc::new(Mutex::new(state)))
    }

    pub fn get(&self) -> MutexGuard<'_, State> {
        self.0.lock().unwrap()
    }

    pub fn get_last(&self) -> u32 {
        self.get().last_swapped
    }

    pub fn set_last(&self, index: u32) -> u32 {
        let mut state = self.get();
        state.last_swapped = index;
        index
    }

    pub fn get_iterations(&self) -> u32 {
        self.get().iterations
    }

    pub fn increment_iterations(&self) -> u32 {
        let mut state = self.get();
        state.iterations += 1;
        state.iterations + 1
    }

    pub fn get_completed(&self) -> bool {
        self.get().completed
    }

    pub fn set_completed(&self, status: bool) -> bool {
        let mut state = self.get();
        state.completed = status;
        status
    }
}

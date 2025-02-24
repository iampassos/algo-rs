use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread,
    time::{Duration, Instant},
};

use crate::app::App;

#[derive(Clone)]
pub enum Status {
    Running,
    Completed,
    Paused,
    Interrupted,
    Checking,
    Failed,
}

#[derive(Clone)]
pub struct State {
    pub array: Vec<u32>,
    pub last_swapped: u32,
    pub comparison: [u32; 2],
    pub checked: Vec<u32>,
    pub iterations: u32,
    pub status: Status,
    pub start: Instant,
    pub end: Instant,
    pub algorithm: String,
}

impl State {
    pub fn new(array: Vec<u32>) -> Self {
        State {
            array,
            iterations: 0,
            last_swapped: 0,
            checked: vec![],
            comparison: [0; 2],
            status: Status::Paused,
            start: Instant::now(),
            end: Instant::now(),
            algorithm: String::from("None"),
        }
    }
}

#[derive(Clone)]
pub struct SharedState(Arc<Mutex<State>>);

impl SharedState {
    pub fn new(state: State) -> Self {
        Self(Arc::new(Mutex::new(state)))
    }

    pub fn reset_array(&self) {
        let array = App::generate_array();
        self.get().array = array;
    }

    pub fn sleep(&self, ms: Option<u64>) {
        thread::sleep(Duration::from_millis(ms.unwrap_or(1)));
    }

    pub fn park(&self) {
        thread::park();
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

    pub fn check(&self) {
        self.set_status(Status::Checking);

        let array = self.get().array.clone();
        let len = array.len();

        for i in 0..len - 1 {
            if i + 1 == len - 1 {
                self.set_checked(u32::try_from(i + 1).unwrap());
            }

            if array[i] < array[i + 1] {
                self.set_checked(u32::try_from(i).unwrap());
                self.set_comparison([u32::try_from(i).unwrap(), u32::try_from(i + 1).unwrap()]);
                self.sleep(Some(5));
            };
        }

        if len == self.get_checked().len() {
            self.set_status(Status::Completed);
        } else {
            self.set_status(Status::Failed);
        }
    }

    pub fn get_checked(&self) -> Vec<u32> {
        self.get().checked.clone()
    }

    pub fn set_checked(&self, index: u32) -> u32 {
        let mut state = self.get();
        state.checked.push(index);
        index
    }

    pub fn get_comparison(&self) -> [u32; 2] {
        self.get().comparison
    }

    pub fn set_comparison(&self, indexes: [u32; 2]) -> [u32; 2] {
        let mut state = self.get();
        state.comparison = indexes;
        indexes
    }

    pub fn get_iterations(&self) -> u32 {
        self.get().iterations
    }

    pub fn increment_iterations(&self) -> u32 {
        let mut state = self.get();
        state.iterations += 1;
        state.iterations + 1
    }

    pub fn get_status(&self) -> Status {
        self.get().status.clone()
    }

    pub fn set_status(&self, status: Status) -> Status {
        let mut state = self.get();
        let clone = status.clone();

        if let Status::Failed | Status::Paused | Status::Checking = clone {
            state.end = Instant::now();
        }

        state.status = clone;
        status
    }

    pub fn get_start(&self) -> Instant {
        self.get().start
    }

    pub fn set_start(&self, start: Instant) -> Instant {
        let mut state = self.get();
        state.start = start;
        start
    }

    pub fn get_algorithm(&self) -> String {
        self.get().algorithm.clone()
    }

    pub fn set_algorithm(&self, name: String) -> String {
        let mut state = self.get();
        let clone = name.clone();
        state.algorithm = clone;
        name
    }

    pub fn get_end(&self) -> Instant {
        self.get().end
    }
}

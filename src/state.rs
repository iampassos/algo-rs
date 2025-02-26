use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread,
    time::Duration,
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
    pub comparisons: u32,
    pub array_accesses: u32,
    pub status: Status,
    pub algorithm: String,
    pub log: Option<String>,
    pub speed: u32,
}

impl State {
    pub fn new(array: Vec<u32>) -> Self {
        State {
            array,
            array_accesses: 0,
            comparisons: 0,
            last_swapped: 999,
            checked: vec![],
            comparison: [999; 2],
            status: Status::Paused,
            algorithm: String::from("None"),
            log: None,
            speed: 100,
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
        thread::sleep(Duration::from_millis(
            ms.unwrap_or((101 - self.get_speed()).into()),
        ));
    }

    pub fn park(&self) {
        thread::park();
    }

    pub fn get(&self) -> MutexGuard<'_, State> {
        self.0.lock().unwrap()
    }

    pub fn get_speed(&self) -> u32 {
        self.get().speed
    }

    pub fn increment_speed(&self) -> u32 {
        let mut state = self.get();
        if state.speed < 100 {
            state.speed += 5;
        }
        state.speed
    }

    pub fn set_speed(&self, value: u32) -> u32 {
        self.get().speed = value;
        self.get_speed()
    }

    pub fn decrement_speed(&self) -> u32 {
        let mut state = self.get();
        if state.speed > 5 {
            state.speed -= 5;
        }
        state.speed
    }

    pub fn get_last(&self) -> u32 {
        self.get().last_swapped
    }

    pub fn set_last(&self, index: u32) -> u32 {
        self.get().last_swapped = index;
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
                self.sleep(None);
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
        self.get().checked.push(index);
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

    pub fn get_accesses(&self) -> u32 {
        self.get().array_accesses
    }

    pub fn increment_accesses(&self, value: u32) -> u32 {
        let mut state = self.get();
        state.array_accesses += value;
        state.array_accesses + value
    }

    pub fn get_comparisons(&self) -> u32 {
        self.get().comparisons
    }

    pub fn increment_comparisons(&self) -> u32 {
        let mut state = self.get();
        state.comparisons += 1;
        state.comparisons
    }

    pub fn get_status(&self) -> Status {
        self.get().status.clone()
    }

    pub fn set_status(&self, status: Status) -> Status {
        let mut state = self.get();
        state.status = status.clone();
        status
    }

    pub fn get_algorithm(&self) -> String {
        self.get().algorithm.clone()
    }

    pub fn log(&self, text: String) -> String {
        self.get().log = Some(text.clone());
        text
    }

    pub fn get_log(&self) -> Option<String> {
        self.get().log.clone()
    }

    pub fn set_algorithm(&self, name: String) -> String {
        self.get().algorithm = name.clone();
        name
    }

    pub fn init_algorithm(&self, name: String) {
        self.set_algorithm(name);
        self.set_status(Status::Paused);
        self.park();
    }
}

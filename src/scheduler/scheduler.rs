use std::collections::BinaryHeap;
use std::time::{Duration, Instant};
use super::task::Task;

pub struct Scheduler {
    tasks: BinaryHeap<Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: BinaryHeap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn run(&mut self) {
        loop {
            if let Some(task) = self.tasks.pop() {
                if task.timer <= Instant::now() {
                    (task.job)();
                } else {
                    // Put back if not ready
                    self.tasks.push(task);
                }
            }
            // Sleep a bit to avoid busy loop
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
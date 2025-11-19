use std::time::{Duration, Instant};

pub struct Task {
    pub id: u32,
    pub priority: u32,
    pub timer: Instant,
    pub job: Box<dyn Fn() + Send + Sync>,
}

impl Task {
    pub fn new(id: u32, priority: u32, delay: Duration, job: Box<dyn Fn() + Send + Sync>) -> Self {
        Task {
            id,
            priority,
            timer: Instant::now() + delay,
            job,
        }
    }
}

impl Eq for Task {}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
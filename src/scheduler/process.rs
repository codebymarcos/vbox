use std::time::{Duration, Instant};

#[derive(Clone, serde::Serialize)]
pub struct ProcessInfo {
    pub id: u32,
    pub priority: u32,
    pub status: String, // e.g., "running", "pending", "completed"
    pub parent_pid: Option<u32>,
    pub memory_usage: usize, // in bytes
}

pub struct Process {
    pub id: u32,
    pub priority: u32,
    pub timer: Instant,
    pub job: Box<dyn FnOnce() + Send + 'static>,
    pub info: ProcessInfo,
}

impl Process {
    pub fn new(
        id: u32,
        priority: u32,
        delay: Duration,
        job: Box<dyn FnOnce() + Send + 'static>,
        parent_pid: Option<u32>,
    ) -> Self {
        let info = ProcessInfo {
            id,
            priority,
            status: "ready".to_string(),
            parent_pid,
            memory_usage: 0, // placeholder
        };
        Process {
            id,
            priority,
            timer: Instant::now() + delay,
            job,
            info,
        }
    }
}

impl Eq for Process {}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

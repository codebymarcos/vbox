use super::process::{Process, ProcessInfo};
use std::collections::BinaryHeap;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

pub struct Scheduler {
    processes: Mutex<BinaryHeap<Process>>,
    all_processes: Mutex<Vec<ProcessInfo>>,
    next_pid: Mutex<u32>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            processes: Mutex::new(BinaryHeap::new()),
            all_processes: Mutex::new(Vec::new()),
            next_pid: Mutex::new(1),
        }
    }

    pub fn add_process(&self, mut process: Process) {
        let mut next_pid = self.next_pid.lock().unwrap();
        process.id = *next_pid;
        let info = process.info.clone();
        self.all_processes.lock().unwrap().push(info);
        *next_pid += 1;
        self.processes.lock().unwrap().push(process);
    }

    pub fn run(&self) {
        let mut handles = vec![];
        while let Some(process) = self.processes.lock().unwrap().pop() {
            if process.timer <= Instant::now() {
                // Update status in all_processes
                if let Some(info) = self
                    .all_processes
                    .lock()
                    .unwrap()
                    .iter_mut()
                    .find(|i| i.id == process.id)
                {
                    info.status = "running".to_string();
                }
                let handle = thread::spawn(move || {
                    (process.job)();
                    // Note: status update not possible here since process is moved
                });
                handles.push(handle);
            } else {
                // Put back
                self.processes.lock().unwrap().push(process);
            }
        }
        // Wait for all
        for handle in handles {
            handle.join().unwrap();
        }
    }

    pub fn list_processes(&self) -> Vec<ProcessInfo> {
        self.all_processes.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_add_process() {
        let scheduler = Scheduler::new();
        let process = Process::new(0, 1, Duration::from_secs(1), Box::new(|| {}), None);
        scheduler.add_process(process);
        let processes = scheduler.list_processes();
        assert_eq!(processes.len(), 1);
        assert_eq!(processes[0].priority, 1);
    }
}

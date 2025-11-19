use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::time::{Duration, Instant};

struct Task {
    id: u32,
    priority: u32,
    timer: Instant,
    job: Box<dyn Fn() + Send + Sync>,
}

impl Task {
    fn new(id: u32, priority: u32, delay: Duration, job: Box<dyn Fn() + Send + Sync>) -> Self {
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

struct Scheduler {
    tasks: BinaryHeap<Reverse<Task>>,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            tasks: BinaryHeap::new(),
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(Reverse(task));
    }

    fn run(&mut self) {
        loop {
            if let Some(Reverse(task)) = self.tasks.pop() {
                if task.timer <= Instant::now() {
                    (task.job)();
                } else {
                    // Put back if not ready
                    self.tasks.push(Reverse(task));
                }
            }
            // Sleep a bit to avoid busy loop
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

fn main() {
    let mut scheduler = Scheduler::new();

    // Add some tasks
    scheduler.add_task(Task::new(1, 1, Duration::from_secs(1), Box::new(|| println!("Task 1 executed"))));
    scheduler.add_task(Task::new(2, 2, Duration::from_secs(2), Box::new(|| println!("Task 2 executed"))));
    scheduler.add_task(Task::new(3, 3, Duration::from_secs(3), Box::new(|| println!("Task 3 executed"))));

    // Run the scheduler in a separate thread
    std::thread::spawn(move || {
        scheduler.run();
    });

    // Main thread does something else
    loop {
        println!("Main loop running...");
        std::thread::sleep(Duration::from_secs(5));
    }
}
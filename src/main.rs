mod dashboard;
mod scheduler;
mod shell;
mod utils;
mod vfs;

use scheduler::{Process, Scheduler};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use vfs::{Disk, FileDisk, FileSystem};

fn main() {
    // Initialize VFS
    let mut fs = FileSystem::new();

    // Create Scheduler
    let scheduler = Arc::new(Scheduler::new());

    // Set /proc
    let proc_dir = Arc::new(crate::vfs::ProcDirectory::new(scheduler.clone()));
    fs.set_proc(proc_dir);

    // Set /dev
    let dev_dir = Arc::new(crate::vfs::DevDirectory::new());
    fs.set_dev(dev_dir);

    // Test Virtual Disk
    let disk = Arc::new(FileDisk::new("vbox_disk.bin"));
    let block_id = disk.allocate_block();
    disk.write_block(block_id, b"Hello Virtual Disk!");
    if let Some(data) = disk.read_block(block_id) {
        println!("Disk initialized: {:?}", String::from_utf8_lossy(&data));
    }

    // Create Shell Process
    let fs_clone = fs.clone();
    let disk_clone = disk.clone();
    let scheduler_clone = scheduler.clone();
    let shell_process = Process::new(
        0,
        1,
        Duration::from_secs(0),
        Box::new(move || {
            let mut shell = shell::Shell::new(fs_clone, disk_clone, scheduler_clone);
            shell.run();
        }),
        None,
    );
    scheduler.add_process(shell_process);

    // Start HTTP Dashboard
    let dashboard_scheduler = scheduler.clone();
    let dashboard_fs = fs.clone();
    let dashboard_disk = disk.clone();
    thread::spawn(move || {
        let dashboard =
            dashboard::HttpDashboard::new(dashboard_scheduler, dashboard_fs, dashboard_disk);
        dashboard.start(8080);
    });

    // Run Scheduler
    scheduler.run();
}

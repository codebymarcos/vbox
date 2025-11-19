mod dashboard;
mod html_renderer;
mod scheduler;
mod shell;
mod utils;
mod vfs;
mod vps;

use scheduler::{Process, Scheduler};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use vfs::{Disk, FileDisk, FileSystem};
use vps::manager::VpsManager;

fn main() {
    // Initialize VFS
    let mut fs = FileSystem::new();

    // Create Scheduler
    let scheduler = Arc::new(Scheduler::new());

    // Create VPS Manager
    let vps_manager = Arc::new(Mutex::new(VpsManager::new()));

    // Set /proc
    let proc_dir = Arc::new(crate::vfs::ProcDirectory::new(scheduler.clone()));
    fs.set_proc(proc_dir);

    // Set /dev
    let dev_dir = Arc::new(crate::vfs::DevDirectory::new());
    fs.set_dev(dev_dir);

    // Set /network
    let network_dir = Arc::new(crate::vfs::NetworkDirectory::new());
    fs.set_network(network_dir);

    // Create /network directory
    fs.create_dir("/network").unwrap();

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
    let vps_manager_clone = vps_manager.clone();
    let shell_process = Process::new(
        0,
        1,
        Duration::from_secs(0),
        Box::new(move || {
            let mut shell = shell::Shell::new(fs_clone, disk_clone, scheduler_clone, vps_manager_clone);
            shell.run();
        }),
        None,
    );
    scheduler.add_process(shell_process);

    // Start HTTP Dashboard
    let dashboard_scheduler = scheduler.clone();
    let dashboard_fs = fs.clone();
    let dashboard_disk = disk.clone();
    let dashboard_vps = vps_manager.clone();
    thread::spawn(move || {
        let dashboard =
            dashboard::HttpDashboard::new(dashboard_scheduler, dashboard_fs, dashboard_disk, dashboard_vps);
        dashboard.start(8080);
    });

    // Run Scheduler
    scheduler.run();
}

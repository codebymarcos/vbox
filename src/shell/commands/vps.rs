use crate::shell::Shell;

pub fn execute(shell: &mut Shell, args: &[&str]) {
    if args.is_empty() {
        println!("Usage: vps <create|list|start|stop|delete> [args...]");
        return;
    }

    match args[0] {
        "create" => {
            if args.len() < 5 {
                println!("Usage: vps create <name> <memory_mb> <disk_mb> <cpu_cores>");
                return;
            }
            let name = args[1];
            let memory_mb = args[2].parse::<usize>().unwrap_or(512);
            let disk_mb = args[3].parse::<usize>().unwrap_or(1024);
            let cpu_cores = args[4].parse::<usize>().unwrap_or(1);

            let mut manager = shell.vps_manager.lock().unwrap();
            match manager.create_vps(name, memory_mb, disk_mb, cpu_cores) {
                Ok(id) => println!("VPS '{}' created with ID: {}", name, id),
                Err(e) => println!("Error creating VPS: {}", e),
            }
        }
        "list" => {
            let manager = shell.vps_manager.lock().unwrap();
            let vps_list = manager.list_vps();
            if vps_list.is_empty() {
                println!("No VPS instances found.");
            } else {
                println!("VPS Instances:");
                println!("{:<36} {:<20} {:<8} {:<8} {:<8} {:<15} {:<10}",
                    "ID", "Name", "Memory", "Disk", "CPU", "IP", "Status");
                println!("{}", "-".repeat(110));
                for vps in vps_list {
                    println!("{:<36} {:<20} {:<8} {:<8} {:<8} {:<15} {:<10}",
                        vps.id, vps.name, format!("{}MB", vps.memory_mb),
                        format!("{}MB", vps.disk_mb), vps.cpu_cores.to_string(),
                        vps.ip_address, vps.status);
                }
            }
        }
        "start" => {
            if args.len() < 2 {
                println!("Usage: vps start <id or name>");
                return;
            }
            let identifier = args[1];
            let mut manager = shell.vps_manager.lock().unwrap();
            match manager.start_vps(identifier) {
                Ok(()) => println!("VPS {} started successfully", identifier),
                Err(e) => println!("Error starting VPS: {}", e),
            }
        }
        "stop" => {
            if args.len() < 2 {
                println!("Usage: vps stop <id or name>");
                return;
            }
            let identifier = args[1];
            let mut manager = shell.vps_manager.lock().unwrap();
            match manager.stop_vps(identifier) {
                Ok(()) => println!("VPS {} stopped successfully", identifier),
                Err(e) => println!("Error stopping VPS: {}", e),
            }
        }
        "delete" => {
            if args.len() < 2 {
                println!("Usage: vps delete <id or name>");
                return;
            }
            let identifier = args[1];
            let mut manager = shell.vps_manager.lock().unwrap();
            match manager.delete_vps(identifier) {
                Ok(()) => println!("VPS {} deleted successfully", identifier),
                Err(e) => println!("Error deleting VPS: {}", e),
            }
        }
        _ => {
            println!("Unknown VPS subcommand: {}", args[0]);
            println!("Available subcommands: create, list, start, stop, delete");
        }
    }
}
use crate::shell::Shell;

pub fn execute(shell: &mut Shell, _args: &[&str]) {
    println!("PID\tPriority\tStatus\tParent PID\tMemory");
    let processes = shell.scheduler.list_processes();
    for process in processes {
        println!(
            "{}\t{}\t{}\t{:?}\t{} bytes",
            process.id, process.priority, process.status, process.parent_pid, process.memory_usage
        );
    }
}

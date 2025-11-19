pub mod browse;
pub mod calc;
pub mod cat;
pub mod cd;
pub mod clearmem;
pub mod echo_cmd;
pub mod ls;
pub mod memory;
pub mod mkdir;
pub mod ps;
pub mod route;
pub mod touch;
pub mod vps;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::Shell;
    use crate::vfs::{FileSystem, Disk};
    use crate::scheduler::Scheduler;
    use std::sync::Arc;

    #[test]
    fn test_calc_command() {
        let fs = FileSystem::new();
        let disk: Arc<dyn Disk + Send + Sync> = Arc::new(crate::vfs::RamDisk::new());
        let scheduler = Arc::new(Scheduler::new());
        let mut shell = Shell::new(fs, disk, scheduler);
        // Capture stdout, but for simplicity, just call and assume no panic
        calc::execute(&mut shell, &["10", "+", "5"]);
        // In real test, check output, but for now, just ensure no panic
    }
}

use crate::shell::Shell;

pub fn execute(shell: &mut Shell, _args: &[&str]) {
    // Clear all blocks in disk
    shell.disk.clear_all();
    println!("Disk memory cleared.");
}

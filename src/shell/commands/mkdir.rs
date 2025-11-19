use crate::shell::Shell;

pub fn execute(shell: &mut Shell, args: &[&str]) {
    if let Some(name) = args.first() {
        let path = shell.resolve_path(name);
        if let Err(e) = shell.fs.create_dir(&path) {
            println!("Error creating directory: {}", e);
        }
    } else {
        println!("Usage: mkdir <directory>");
    }
}

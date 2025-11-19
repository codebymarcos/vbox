use crate::shell::Shell;
use crate::vfs::NodeType;

pub fn execute(shell: &mut Shell, args: &[&str]) {
    if let Some(path) = args.first() {
        let full_path = shell.resolve_path(path);
        if let Some(node) = shell.fs.get(&full_path) {
            if node.node_type() == NodeType::Directory {
                shell.cwd = full_path;
            } else {
                println!("{} is not a directory", full_path);
            }
        } else {
            println!("Directory not found: {}", full_path);
        }
    } else {
        println!("Usage: cd <directory>");
    }
}

use crate::shell::Shell;
use crate::vfs::NodeType;

pub fn execute(shell: &mut Shell, args: &[&str]) {
    if let Some(name) = args.first() {
        let path = shell.resolve_path(name);
        if let Some(node) = shell.fs.get(&path) {
            if node.node_type() == NodeType::File {
                // Check if it's a regular file
                if let Some(file) = node.as_any().downcast_ref::<crate::vfs::File>() {
                    println!("{}", String::from_utf8_lossy(&file.read()));
                } else if let Some(null_dev) =
                    node.as_any().downcast_ref::<crate::vfs::NullDevice>()
                {
                    println!("{}", null_dev.read());
                } else if let Some(random_dev) =
                    node.as_any().downcast_ref::<crate::vfs::RandomDevice>()
                {
                    println!("{}", random_dev.read());
                } else {
                    println!("Unsupported file type");
                }
            } else {
                println!("{} is not a file", path);
            }
        } else {
            println!("File not found: {}", path);
        }
    } else {
        println!("Usage: cat <file>");
    }
}

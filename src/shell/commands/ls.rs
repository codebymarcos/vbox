use crate::shell::Shell;
use crate::vfs::NodeType;

pub fn execute(shell: &mut Shell, args: &[&str]) {
    let path = args
        .first()
        .map(|s| shell.resolve_path(s))
        .unwrap_or_else(|| shell.cwd.clone());
    if let Some(node) = shell.fs.get(&path) {
        if node.node_type() == NodeType::Directory {
            let names = if let Some(dir) = node.as_any().downcast_ref::<crate::vfs::Directory>() {
                dir.list()
            } else if let Some(proc_dir) = node.as_any().downcast_ref::<crate::vfs::ProcDirectory>()
            {
                proc_dir.list()
            } else {
                vec![]
            };
            for name in names {
                println!("{}", name);
            }
        } else {
            println!("{} is not a directory", path);
        }
    } else {
        println!("Path not found: {}", path);
    }
}

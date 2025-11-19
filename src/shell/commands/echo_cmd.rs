use crate::shell::Shell;
use crate::vfs::NodeType;

pub fn execute(shell: &mut Shell, args: &[&str]) {
    if args.len() >= 3 && args[args.len() - 2] == ">" {
        let text = args[..args.len() - 2].join(" ");
        let file_path = shell.resolve_path(args[args.len() - 1]);
        if let Some(node) = shell.fs.get(&file_path) {
            if node.node_type() == NodeType::File {
                let file = node.as_any().downcast_ref::<crate::vfs::File>().unwrap();
                file.write(text.as_bytes());
            } else {
                println!("{} is not a file", file_path);
            }
        } else {
            println!("File not found: {}", file_path);
        }
    } else {
        println!("Usage: echo <text> > <file>");
    }
}

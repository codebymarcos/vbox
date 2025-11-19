use crate::shell::Shell;
use crate::vfs::NodeType;

pub fn execute(shell: &mut Shell, args: &[&str]) {
    if args.is_empty() {
        println!("Usage: route <add|list> [destination] [gateway]");
        return;
    }

    match args[0] {
        "list" => {
            if let Some(node) = shell.fs.get("/network") {
                if node.node_type() == NodeType::Directory {
                    if let Some(dir) = node.as_any().downcast_ref::<crate::vfs::NetworkDirectory>() {
                        let routes = dir.get_routes();
                        for (dest, gateway) in routes {
                            println!("{} -> {}", dest, gateway);
                        }
                    }
                }
            }
        }
        "add" => {
            if args.len() < 3 {
                println!("Usage: route add <destination> <gateway>");
                return;
            }
            let destination = args[1];
            let gateway = args[2];
            if let Some(node) = shell.fs.get("/network") {
                if node.node_type() == NodeType::Directory {
                    if let Some(dir) = node.as_any().downcast_ref::<crate::vfs::NetworkDirectory>() {
                        dir.add_route(destination, gateway);
                        println!("Route added: {} -> {}", destination, gateway);
                    }
                }
            }
        }
        _ => {
            println!("Unknown subcommand: {}", args[0]);
        }
    }
}
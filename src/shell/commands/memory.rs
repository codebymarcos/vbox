use crate::shell::Shell;

pub fn execute(shell: &mut Shell, _args: &[&str]) {
    // Show RAM disk memory usage
    let num_blocks = shell.disk.get_allocated_blocks();
    let total_size = shell.disk.get_total_size();
    println!("Disk Memory:");
    println!("  Blocks allocated: {}", num_blocks);
    println!("  Total data size: {} bytes", total_size);

    // VFS stats
    println!("VFS Stats:");
    let mut file_count = 0;
    let mut dir_count = 0;
    let mut total_file_size = 0;
    fn count_nodes(
        node: &dyn crate::vfs::VfsNode,
        files: &mut u32,
        dirs: &mut u32,
        size: &mut usize,
    ) {
        match node.node_type() {
            crate::vfs::NodeType::File => {
                *files += 1;
                if let Some(file) = node.as_any().downcast_ref::<crate::vfs::File>() {
                    *size += file.read().len();
                }
            }
            crate::vfs::NodeType::Directory => {
                *dirs += 1;
                if let Some(dir) = node.as_any().downcast_ref::<crate::vfs::Directory>() {
                    for child in dir.list() {
                        if let Some(child_node) = dir.get(&child) {
                            count_nodes(&*child_node, files, dirs, size);
                        }
                    }
                }
            }
        }
    }
    if let Some(root) = shell.fs.get("/") {
        count_nodes(
            &*root,
            &mut file_count,
            &mut dir_count,
            &mut total_file_size,
        );
    }
    println!("  Directories: {}", dir_count);
    println!("  Files: {}", file_count);
    println!("  Total file data: {} bytes", total_file_size);

    println!("Note: All data is in-memory.");
}

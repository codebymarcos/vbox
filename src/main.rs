mod scheduler;
mod utils;
mod vfs;

use vfs::FileSystem;

fn main() {
    let fs = FileSystem::new();

    fs.create_dir("/home").unwrap();
    let file = fs.create_file("/home/test.txt").unwrap();

    file.write(b"Hello VFS!");
    println!("{:?}", String::from_utf8_lossy(&file.read()));
}

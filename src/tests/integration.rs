#[cfg(test)]
mod integration_tests {
    use std::fs;
    use std::sync::Arc;
    use crate::vfs::{FileDisk, Disk};

    #[test]
    fn test_file_disk_persistence() {
        let disk_path = "integration_test_disk.bin";
        // Clean up
        let _ = fs::remove_file(disk_path);

        // Create disk
        let disk: Arc<dyn Disk + Send + Sync> = Arc::new(FileDisk::new(disk_path));

        // Write data
        let block_id = disk.allocate_block();
        disk.write_block(block_id, b"Hello, persistent world!");

        // Drop to save
        drop(disk);

        // Recreate disk
        let disk2: Arc<dyn Disk + Send + Sync> = Arc::new(FileDisk::new(disk_path));

        // Read data back
        let data = disk2.read_block(block_id).unwrap();
        assert_eq!(data, b"Hello, persistent world!");

        // Clean up
        fs::remove_file(disk_path).unwrap();
    }
}
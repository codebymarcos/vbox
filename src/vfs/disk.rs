use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(serde::Serialize, serde::Deserialize)]
struct DiskData {
    blocks: HashMap<u64, Vec<u8>>,
    checksum: u64,
}

pub trait Disk: Send + Sync {
    fn read_block(&self, block_id: u64) -> Option<Vec<u8>>;
    fn write_block(&self, block_id: u64, data: &[u8]);
    fn allocate_block(&self) -> u64;
    fn free_block(&self, block_id: u64);
    fn get_allocated_blocks(&self) -> usize;
    fn get_total_size(&self) -> usize;
    fn clear_all(&self);
}

#[derive(Clone)]
pub struct RamDisk {
    pub(crate) blocks: Arc<Mutex<HashMap<u64, Vec<u8>>>>,
    pub(crate) next_block: Arc<Mutex<u64>>,
}

impl RamDisk {
    pub fn new() -> Self {
        RamDisk {
            blocks: Arc::new(Mutex::new(HashMap::new())),
            next_block: Arc::new(Mutex::new(0)),
        }
    }
}

impl Disk for RamDisk {
    fn read_block(&self, block_id: u64) -> Option<Vec<u8>> {
        self.blocks.lock().unwrap().get(&block_id).cloned()
    }

    fn write_block(&self, block_id: u64, data: &[u8]) {
        self.blocks.lock().unwrap().insert(block_id, data.to_vec());
    }

    fn allocate_block(&self) -> u64 {
        let mut next = self.next_block.lock().unwrap();
        let id = *next;
        *next += 1;
        id
    }

    fn free_block(&self, block_id: u64) {
        self.blocks.lock().unwrap().remove(&block_id);
    }

    fn get_allocated_blocks(&self) -> usize {
        self.blocks.lock().unwrap().len()
    }

    fn get_total_size(&self) -> usize {
        self.blocks.lock().unwrap().values().map(|v| v.len()).sum()
    }

    fn clear_all(&self) {
        self.blocks.lock().unwrap().clear();
        *self.next_block.lock().unwrap() = 0;
    }
}

#[derive(Clone)]
pub struct FileDisk {
    pub(crate) blocks: Arc<Mutex<HashMap<u64, Vec<u8>>>>,
    pub(crate) next_block: Arc<Mutex<u64>>,
    pub(crate) file_path: String,
}

impl FileDisk {
    pub fn new(file_path: &str) -> Self {
        let path = Path::new(file_path);
        let (blocks, next_block) = if path.exists() {
            // Load from file
            let data = fs::read(path).unwrap_or_default();
            if let Ok(disk_data) = bincode::deserialize::<DiskData>(&data) {
                // Verify checksum
                let serialized_blocks = bincode::serialize(&disk_data.blocks).unwrap();
                let mut hasher = DefaultHasher::new();
                serialized_blocks.hash(&mut hasher);
                if hasher.finish() == disk_data.checksum {
                    let next_block = disk_data.blocks.keys().max().map(|&k| k + 1).unwrap_or(0);
                    (disk_data.blocks, next_block)
                } else {
                    eprintln!("Checksum mismatch, initializing empty disk");
                    (HashMap::new(), 0)
                }
            } else {
                (HashMap::new(), 0)
            }
        } else {
            (HashMap::new(), 0)
        };
        FileDisk {
            blocks: Arc::new(Mutex::new(blocks)),
            next_block: Arc::new(Mutex::new(next_block)),
            file_path: file_path.to_string(),
        }
    }

    fn save(&self) {
        let blocks = self.blocks.lock().unwrap();
        let serialized_blocks = bincode::serialize(&*blocks).unwrap();
        let mut hasher = DefaultHasher::new();
        serialized_blocks.hash(&mut hasher);
        let checksum = hasher.finish();
        let disk_data = DiskData {
            blocks: blocks.clone(),
            checksum,
        };
        let data = bincode::serialize(&disk_data).unwrap();
        fs::write(&self.file_path, data).unwrap();
    }
}

impl Disk for FileDisk {
    fn read_block(&self, block_id: u64) -> Option<Vec<u8>> {
        self.blocks.lock().unwrap().get(&block_id).cloned()
    }

    fn write_block(&self, block_id: u64, data: &[u8]) {
        {
            self.blocks.lock().unwrap().insert(block_id, data.to_vec());
        }
        self.save();
    }

    fn allocate_block(&self) -> u64 {
        let mut next = self.next_block.lock().unwrap();
        let id = *next;
        *next += 1;
        id
    }

    fn free_block(&self, block_id: u64) {
        {
            self.blocks.lock().unwrap().remove(&block_id);
        }
        self.save();
    }

    fn get_allocated_blocks(&self) -> usize {
        self.blocks.lock().unwrap().len()
    }

    fn get_total_size(&self) -> usize {
        self.blocks.lock().unwrap().values().map(|v| v.len()).sum()
    }

    fn clear_all(&self) {
        self.blocks.lock().unwrap().clear();
        *self.next_block.lock().unwrap() = 0;
        self.save();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_disk() {
        let disk = RamDisk::new();
        let block_id = disk.allocate_block();
        disk.write_block(block_id, b"test data");
        let data = disk.read_block(block_id).unwrap();
        assert_eq!(data, b"test data");
        disk.free_block(block_id);
        assert!(disk.read_block(block_id).is_none());
    }

    #[test]
    fn test_file_disk() {
        let path = "test_disk.bin";
        let disk = FileDisk::new(path);
        let block_id = disk.allocate_block();
        disk.write_block(block_id, b"file test");
        let data = disk.read_block(block_id).unwrap();
        assert_eq!(data, b"file test");
        disk.free_block(block_id);
        fs::remove_file(path).unwrap();
    }
}

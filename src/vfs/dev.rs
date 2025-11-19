use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use super::{DirectoryOps, NodeType, VfsNode};

pub struct DevDirectory {
    devices: HashMap<String, Arc<dyn VfsNode>>,
}

impl DevDirectory {
    pub fn new() -> Self {
        let mut devices = HashMap::new();

        // Add /dev/null
        let null_dev: Arc<dyn VfsNode> = Arc::new(NullDevice::new());
        devices.insert("null".to_string(), null_dev);

        // Add /dev/random (simple random)
        let random_dev: Arc<dyn VfsNode> = Arc::new(RandomDevice::new());
        devices.insert("random".to_string(), random_dev);

        DevDirectory { devices }
    }
}

impl VfsNode for DevDirectory {
    fn name(&self) -> String {
        "dev".to_string()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Directory
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl DirectoryOps for DevDirectory {
    fn list(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }

    fn get(&self, name: &str) -> Option<Arc<dyn VfsNode>> {
        self.devices.get(name).cloned()
    }

    fn add(&self, _node: Arc<dyn VfsNode>) {
        // Read-only
    }
}

// /dev/null - discards all writes, reads return empty
pub struct NullDevice;

impl NullDevice {
    pub fn new() -> Self {
        NullDevice
    }

    pub fn read(&self) -> String {
        String::new()
    }

    pub fn write(&mut self, _data: &str) {
        // Discard
    }
}

impl VfsNode for NullDevice {
    fn name(&self) -> String {
        "null".to_string()
    }

    fn node_type(&self) -> NodeType {
        NodeType::File
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// /dev/random - returns random bytes
pub struct RandomDevice;

impl RandomDevice {
    pub fn new() -> Self {
        RandomDevice
    }

    pub fn read(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("{}", time % 256) // Simple "random"
    }

    pub fn write(&mut self, _data: &str) {
        // Ignore writes
    }
}

impl VfsNode for RandomDevice {
    fn name(&self) -> String {
        "random".to_string()
    }

    fn node_type(&self) -> NodeType {
        NodeType::File
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

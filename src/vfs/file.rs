use super::{NodeType, VfsNode};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub content: Arc<Mutex<Vec<u8>>>,
}

impl File {
    pub fn new(name: &str) -> Self {
        File {
            name: name.to_string(),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn write(&self, data: &[u8]) {
        let mut lock = self.content.lock().unwrap();
        lock.extend_from_slice(data);
    }

    pub fn read(&self) -> Vec<u8> {
        let lock = self.content.lock().unwrap();
        lock.clone()
    }
}

impl VfsNode for File {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn node_type(&self) -> NodeType {
        NodeType::File
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

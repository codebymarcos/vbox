use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{DirectoryOps, NodeType, VfsNode};

#[derive(Clone)]
pub struct Directory {
    pub name: String,
    pub children: Arc<Mutex<HashMap<String, Arc<dyn VfsNode>>>>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            children: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add(&self, node: Arc<dyn VfsNode>) {
        self.children.lock().unwrap().insert(node.name(), node);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn VfsNode>> {
        self.children.lock().unwrap().get(name).cloned()
    }

    pub fn list(&self) -> Vec<String> {
        self.children.lock().unwrap().keys().cloned().collect()
    }
}

impl DirectoryOps for Directory {
    fn list(&self) -> Vec<String> {
        self.list()
    }

    fn get(&self, name: &str) -> Option<Arc<dyn VfsNode>> {
        self.get(name)
    }

    fn add(&self, node: Arc<dyn VfsNode>) {
        self.add(node);
    }
}

impl VfsNode for Directory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Directory
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

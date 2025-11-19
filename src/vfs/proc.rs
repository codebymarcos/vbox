use std::any::Any;
use std::sync::Arc;

use super::{DirectoryOps, NodeType, VfsNode};

pub struct ProcDirectory {
    pub scheduler: Arc<crate::scheduler::Scheduler>,
}

impl ProcDirectory {
    pub fn new(scheduler: Arc<crate::scheduler::Scheduler>) -> Self {
        ProcDirectory { scheduler }
    }

    pub fn add(&self, _node: Arc<dyn VfsNode>) {
        // Read-only
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn VfsNode>> {
        if let Ok(id) = name.parse::<u32>() {
            if let Some(process) = self.scheduler.list_processes().iter().find(|p| p.id == id) {
                // Return a ProcFile with process info
                Some(Arc::new(ProcFile {
                    content: format!("PID: {}\nPriority: {}\nStatus: {}\nParent PID: {:?}\nMemory Usage: {} bytes\n", 
                                     process.id, process.priority, process.status, process.parent_pid, process.memory_usage),
                }))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn list(&self) -> Vec<String> {
        // List process IDs as files
        self.scheduler
            .list_processes()
            .iter()
            .map(|p| p.id.to_string())
            .collect()
    }
}

impl DirectoryOps for ProcDirectory {
    fn list(&self) -> Vec<String> {
        self.list()
    }

    fn get(&self, name: &str) -> Option<Arc<dyn VfsNode>> {
        self.get(name)
    }

    fn add(&self, _node: Arc<dyn VfsNode>) {
        // Read-only
    }
}

impl VfsNode for ProcDirectory {
    fn name(&self) -> String {
        "proc".to_string()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Directory
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ProcFile {
    content: String,
}

impl ProcFile {
    pub fn read(&self) -> String {
        self.content.clone()
    }

    pub fn write(&mut self, _data: &str) {
        // Read-only
    }
}

impl VfsNode for ProcFile {
    fn name(&self) -> String {
        "task".to_string()
    }

    fn node_type(&self) -> NodeType {
        NodeType::File
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

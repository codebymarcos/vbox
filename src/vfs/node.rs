use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    File,
    Directory,
}

pub trait VfsNode: Send + Sync {
    fn name(&self) -> String;
    fn node_type(&self) -> NodeType;
    fn as_any(&self) -> &dyn Any;
}

pub trait DirectoryOps {
    fn list(&self) -> Vec<String>;
    fn get(&self, name: &str) -> Option<Arc<dyn VfsNode>>;
    fn add(&self, node: Arc<dyn VfsNode>);
}

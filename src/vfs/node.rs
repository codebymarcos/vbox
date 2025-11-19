use std::any::Any;

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
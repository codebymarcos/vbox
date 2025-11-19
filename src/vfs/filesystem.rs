use std::sync::Arc;

use super::{Directory, File, VfsNode, NodeType};

pub struct FileSystem {
    pub root: Arc<Directory>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            root: Arc::new(Directory::new("/")),
        }
    }

    fn split_path(path: &str) -> Vec<&str> {
        path.trim_matches('/')
            .split('/')
            .filter(|p| !p.is_empty())
            .collect()
    }

    pub fn create_file(&self, path: &str) -> Result<Arc<File>, String> {
        let (parent, name) = self.resolve_parent(path)?;
        let file = Arc::new(File::new(name));
        parent.add(file.clone());
        Ok(file)
    }

    pub fn create_dir(&self, path: &str) -> Result<Arc<Directory>, String> {
        let (parent, name) = self.resolve_parent(path)?;
        let dir = Arc::new(Directory::new(name));
        parent.add(dir.clone());
        Ok(dir)
    }

    pub fn get(&self, path: &str) -> Option<Arc<dyn VfsNode>> {
        if path == "/" {
            return Some(self.root.clone());
        }

        let parts = Self::split_path(path);
        let mut current: Arc<dyn VfsNode> = self.root.clone();

        for part in parts {
            if current.node_type() != NodeType::Directory {
                return None;
            }
            let dir = current.as_any().downcast_ref::<Directory>().unwrap();
            current = dir.get(part)?;
        }
        Some(current)
    }

    fn resolve_parent(&self, path: &str) -> Result<(Arc<Directory>, &str), String> {
        let mut parts = Self::split_path(path);
        let name = parts.pop().ok_or("Invalid path")?;

        let parent_path = if parts.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", parts.join("/"))
        };

        let parent = self
            .get(&parent_path)
            .ok_or("Parent not found")?;

        if parent.node_type() != NodeType::Directory {
            return Err("Parent is not directory".into());
        }

        let dir = parent.as_any().downcast_ref::<Directory>().unwrap().clone();
        Ok((Arc::new(dir), name))
    }
}
use std::sync::Arc;

use super::{Directory, DirectoryOps, File, NodeType, VfsNode};

#[derive(Clone)]
pub struct FileSystem {
    pub root: Arc<Directory>,
    pub proc_dir: Option<Arc<dyn VfsNode>>,
    pub dev_dir: Option<Arc<dyn VfsNode>>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            root: Arc::new(Directory::new("/")),
            proc_dir: None,
            dev_dir: None,
        }
    }

    pub fn set_proc(&mut self, proc: Arc<dyn VfsNode>) {
        self.proc_dir = Some(proc);
    }

    pub fn set_dev(&mut self, dev: Arc<dyn VfsNode>) {
        self.dev_dir = Some(dev);
    }

    fn split_path(path: &str) -> Vec<&str> {
        path.trim_matches('/')
            .split('/')
            .filter(|p| !p.is_empty())
            .collect()
    }

    pub fn create_file(&self, path: &str) -> Result<Arc<File>, String> {
        let (parent, name) = self.resolve_parent(path)?;
        let file = Arc::new(File::new(&name));
        parent.add(file.clone());
        Ok(file)
    }

    pub fn create_dir(&self, path: &str) -> Result<Arc<Directory>, String> {
        let (parent, name) = self.resolve_parent(path)?;
        let dir = Arc::new(Directory::new(&name));
        parent.add(dir.clone());
        Ok(dir)
    }

    pub fn get(&self, path: &str) -> Option<Arc<dyn VfsNode>> {
        if path == "/" {
            return Some(self.root.clone());
        }
        if path == "/proc" {
            if let Some(proc) = &self.proc_dir {
                return Some(proc.clone());
            }
        }
        if path.starts_with("/proc/") {
            if let Some(proc) = &self.proc_dir {
                let name = path.strip_prefix("/proc/").unwrap();
                if let Some(dir) = proc.as_any().downcast_ref::<crate::vfs::ProcDirectory>() {
                    return dir.get(name);
                }
            }
        }
        if path == "/dev" {
            if let Some(dev) = &self.dev_dir {
                return Some(dev.clone());
            }
        }
        if path.starts_with("/dev/") {
            if let Some(dev) = &self.dev_dir {
                let name = path.strip_prefix("/dev/").unwrap();
                if let Some(dir) = dev.as_any().downcast_ref::<crate::vfs::DevDirectory>() {
                    return dir.get(name);
                }
            }
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

    fn resolve_parent(&self, path: &str) -> Result<(Arc<Directory>, String), String> {
        let mut parts = Self::split_path(path);
        let name = parts.pop().ok_or("Invalid path")?;

        let parent_path = if parts.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", parts.join("/"))
        };

        let parent = self.get(&parent_path).ok_or("Parent not found")?;

        if parent.node_type() != NodeType::Directory {
            return Err("Parent is not directory".into());
        }

        let dir = parent.as_any().downcast_ref::<Directory>().unwrap().clone();
        Ok((Arc::new(dir), name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() {
        let fs = FileSystem::new();
        let file = fs.create_file("/test.txt").unwrap();
        assert_eq!(file.name(), "test.txt");
        assert!(fs.get("/test.txt").is_some());
    }

    #[test]
    fn test_create_dir() {
        let fs = FileSystem::new();
        let dir = fs.create_dir("/testdir").unwrap();
        assert_eq!(dir.name(), "testdir");
        assert!(fs.get("/testdir").is_some());
    }

    #[test]
    fn test_split_path() {
        assert_eq!(FileSystem::split_path("/"), Vec::<&str>::new());
        assert_eq!(FileSystem::split_path("/a"), vec!["a"]);
        assert_eq!(FileSystem::split_path("/a/b/c"), vec!["a", "b", "c"]);
    }
}

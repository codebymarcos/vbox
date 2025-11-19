use super::{DirectoryOps, File, NodeType, VfsNode};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct NetworkDirectory {
    routes: Arc<Mutex<HashMap<String, String>>>,
}

impl NetworkDirectory {
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        routes.insert("default".to_string(), "0.0.0.0/0 via 192.168.1.1".to_string());
        NetworkDirectory {
            routes: Arc::new(Mutex::new(routes)),
        }
    }

    pub fn add_route(&self, destination: &str, gateway: &str) {
        let mut routes = self.routes.lock().unwrap();
        routes.insert(destination.to_string(), gateway.to_string());
    }

    pub fn get_routes(&self) -> HashMap<String, String> {
        self.routes.lock().unwrap().clone()
    }
}

impl DirectoryOps for NetworkDirectory {
    fn list(&self) -> Vec<String> {
        let routes = self.routes.lock().unwrap();
        routes.keys().cloned().collect()
    }

    fn get(&self, name: &str) -> Option<Arc<dyn VfsNode>> {
        let routes = self.routes.lock().unwrap();
        if let Some(route) = routes.get(name) {
            let content = format!("Destination: {}\nGateway: {}", name, route);
            Some(Arc::new(File::new_with_content(name, content)))
        } else {
            None
        }
    }

    fn add(&self, _node: Arc<dyn VfsNode>) {
        // Not implemented for routes
    }
}

impl VfsNode for NetworkDirectory {
    fn name(&self) -> String {
        "network".to_string()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Directory
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
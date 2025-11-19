use super::instance::{VpsInstance, VpsConfig};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct VpsManager {
    vps_instances: HashMap<String, Arc<Mutex<VpsInstance>>>,
}

impl VpsManager {
    pub fn new() -> Self {
        VpsManager {
            vps_instances: HashMap::new(),
        }
    }

    pub fn create_vps(&mut self, name: &str, memory_mb: usize, disk_mb: usize, cpu_cores: usize) -> Result<String, String> {
        let id = Uuid::new_v4().to_string();

        // Gerar IP automaticamente (simples para demonstração)
        let ip_address = format!("192.168.1.{}", 100 + self.vps_instances.len());

        let config = VpsConfig {
            id: id.clone(),
            name: name.to_string(),
            memory_mb,
            disk_mb,
            cpu_cores,
            ip_address,
            status: "stopped".to_string(),
        };

        let instance = Arc::new(Mutex::new(VpsInstance::new(config)));
        self.vps_instances.insert(id.clone(), instance);

        Ok(id)
    }

    pub fn start_vps(&mut self, identifier: &str) -> Result<(), String> {
        // Try to find by ID first, then by name
        let vps_id = if self.vps_instances.contains_key(identifier) {
            identifier.to_string()
        } else {
            // Look for VPS by name
            let mut found_id = None;
            for (id, instance) in &self.vps_instances {
                if instance.lock().unwrap().config.name == identifier {
                    found_id = Some(id.clone());
                    break;
                }
            }
            match found_id {
                Some(id) => id,
                None => return Err(format!("VPS {} not found", identifier)),
            }
        };

        if let Some(instance) = self.vps_instances.get(&vps_id) {
            instance.lock().unwrap().start()
        } else {
            Err(format!("VPS {} not found", identifier))
        }
    }

    pub fn stop_vps(&mut self, identifier: &str) -> Result<(), String> {
        // Try to find by ID first, then by name
        let vps_id = if self.vps_instances.contains_key(identifier) {
            identifier.to_string()
        } else {
            // Look for VPS by name
            let mut found_id = None;
            for (id, instance) in &self.vps_instances {
                if instance.lock().unwrap().config.name == identifier {
                    found_id = Some(id.clone());
                    break;
                }
            }
            match found_id {
                Some(id) => id,
                None => return Err(format!("VPS {} not found", identifier)),
            }
        };

        if let Some(instance) = self.vps_instances.get(&vps_id) {
            instance.lock().unwrap().stop()
        } else {
            Err(format!("VPS {} not found", identifier))
        }
    }

    pub fn list_vps(&self) -> Vec<VpsConfig> {
        self.vps_instances.values()
            .map(|instance| instance.lock().unwrap().get_info())
            .collect()
    }

    pub fn get_vps(&self, id: &str) -> Option<Arc<Mutex<VpsInstance>>> {
        self.vps_instances.get(id).cloned()
    }

    pub fn delete_vps(&mut self, identifier: &str) -> Result<(), String> {
        // Try to find by ID first, then by name
        let vps_id = if self.vps_instances.contains_key(identifier) {
            identifier.to_string()
        } else {
            // Look for VPS by name
            let mut found_id = None;
            for (id, instance) in &self.vps_instances {
                if instance.lock().unwrap().config.name == identifier {
                    found_id = Some(id.clone());
                    break;
                }
            }
            match found_id {
                Some(id) => id,
                None => return Err(format!("VPS {} not found", identifier)),
            }
        };

        if let Some(instance) = self.vps_instances.remove(&vps_id) {
            let mut instance = instance.lock().unwrap();
            if instance.get_status() == "running" {
                instance.stop()?;
            }
            // TODO: Limpar arquivos do disco
            Ok(())
        } else {
            Err(format!("VPS {} not found", identifier))
        }
    }
}
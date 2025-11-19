use crate::vfs::{FileSystem, Disk, FileDisk};
use crate::scheduler::{Scheduler, Process};
use crate::vps::manager::VpsManager;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone, Debug, serde::Serialize)]
pub struct VpsConfig {
    pub id: String,
    pub name: String,
    pub memory_mb: usize,
    pub disk_mb: usize,
    pub cpu_cores: usize,
    pub ip_address: String,
    pub status: String, // "stopped", "running", "paused"
}

pub struct VpsInstance {
    pub config: VpsConfig,
    pub filesystem: Arc<Mutex<FileSystem>>,
    pub disk: Arc<dyn Disk + Send + Sync>,
    pub scheduler: Arc<Scheduler>,
    pub processes: Vec<u32>, // PIDs dos processos desta VPS
}

impl VpsInstance {
    pub fn new(config: VpsConfig) -> Self {
        // Criar filesystem isolado para a VPS
        let mut fs = FileSystem::new();

        // Criar disco virtual para a VPS
        let disk_path = format!("vps_{}_disk.bin", config.id);
        let disk = Arc::new(FileDisk::new(&disk_path));

        // Inicializar disco
        let block_id = disk.allocate_block();
        disk.write_block(block_id, format!("VPS {} Disk", config.name).as_bytes());

        // Criar scheduler dedicado para a VPS
        let scheduler = Arc::new(Scheduler::new());

        // Configurar diretórios básicos
        let proc_dir = Arc::new(crate::vfs::ProcDirectory::new(scheduler.clone()));
        fs.set_proc(proc_dir);

        let dev_dir = Arc::new(crate::vfs::DevDirectory::new());
        fs.set_dev(dev_dir);

        let network_dir = Arc::new(crate::vfs::NetworkDirectory::new());
        fs.set_network(network_dir);

        // Criar diretórios básicos
        fs.create_dir("/network").unwrap();
        fs.create_dir("/home").unwrap();
        fs.create_dir("/etc").unwrap();

        VpsInstance {
            config,
            filesystem: Arc::new(Mutex::new(fs)),
            disk,
            scheduler,
            processes: Vec::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.config.status == "running" {
            return Err("VPS already running".to_string());
        }

        self.config.status = "running".to_string();

        // Criar processo shell para a VPS
        let fs_clone = self.filesystem.clone();
        let disk_clone = self.disk.clone();
        let scheduler_clone = self.scheduler.clone();

        let shell_process = Process::new(
            0,
            1,
            Duration::from_secs(0),
            Box::new(move || {
                let mut shell = crate::shell::Shell::new(
                    (*fs_clone.lock().unwrap()).clone(),
                    disk_clone,
                    scheduler_clone,
                    Arc::new(Mutex::new(VpsManager::new())), // VPS isolada sem VpsManager próprio
                );
                shell.run();
            }),
            None,
        );

        let pid = shell_process.id;
        self.scheduler.add_process(shell_process);
        self.processes.push(pid);

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if self.config.status == "stopped" {
            return Err("VPS already stopped".to_string());
        }

        self.config.status = "stopped".to_string();
        // TODO: Implementar parada graciosa dos processos
        Ok(())
    }

    pub fn get_status(&self) -> &str {
        &self.config.status
    }

    pub fn get_info(&self) -> VpsConfig {
        self.config.clone()
    }
}
use crate::scheduler::Scheduler;
use crate::vfs::{Disk, FileSystem};
use crate::vps::manager::VpsManager;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

mod commands;

pub struct Shell {
    fs: FileSystem,
    disk: Arc<dyn Disk + Send + Sync>,
    scheduler: Arc<Scheduler>,
    vps_manager: Arc<Mutex<VpsManager>>,
    cwd: String,
}

impl Shell {
    pub fn new(
        fs: FileSystem,
        disk: Arc<dyn Disk + Send + Sync>,
        scheduler: Arc<Scheduler>,
        vps_manager: Arc<Mutex<VpsManager>>,
    ) -> Self {
        Shell {
            fs,
            disk,
            scheduler,
            vps_manager,
            cwd: "/".to_string(),
        }
    }

    pub fn run(&mut self) {
        println!("VBOX Shell - Type 'help' for commands, 'exit' to quit.");
        loop {
            print!("{}> ", self.cwd);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input.is_empty() {
                continue;
            }
            if self.execute(input) {
                break;
            }
        }
    }

    fn execute(&mut self, command: &str) -> bool {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }
        let args = &parts[1..];
        match parts[0] {
            "exit" => return true,
            "help" => self.help(),
            "clear" => self.clear(),
            "calc" => commands::calc::execute(self, args),
            "ls" => commands::ls::execute(self, args),
            "cd" => commands::cd::execute(self, args),
            "mkdir" => commands::mkdir::execute(self, args),
            "touch" => commands::touch::execute(self, args),
            "cat" => commands::cat::execute(self, args),
            "echo" => commands::echo_cmd::execute(self, args),
            "mem" | "memory" => commands::memory::execute(self, args),
            "clearmem" | "freeram" => commands::clearmem::execute(self, args),
            "ps" => commands::ps::execute(self, args),
            "route" => commands::route::execute(self, args),
            "browse" => commands::browse::execute(self, args),
            "vps" => commands::vps::execute(self, args),
            _ => println!("Unknown command: {}", parts[0]),
        }
        false
    }

    fn help(&self) {
        println!("Commands:");
        println!("  ls [dir]     - List directory contents");
        println!("  cd <dir>     - Change directory");
        println!("  mkdir <dir>  - Create directory");
        println!("  touch <file> - Create file");
        println!("  cat <file>   - Display file contents");
        println!("  echo <text> > <file> - Write text to file");
        println!("  mem/memory   - Show memory usage");
        println!("  clearmem/freeram - Clear RAM memory");
        println!("  ps           - List running tasks");
        println!("  calc <num1> <op> <num2> - Simple calculator");
        println!("  route <list|add> - Manage network routes");
        println!("  browse <url> - Browse web pages");
        println!("  vps <create|list|start|stop|delete> - Manage virtual private servers");
        println!("  clear        - Clear the screen");
        println!("  help         - Show this help");
        println!("  exit         - Exit shell");
    }

    fn clear(&self) {
        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().flush().unwrap();
    }

    pub fn resolve_path(&self, path: &str) -> String {
        if path.starts_with('/') {
            path.to_string()
        } else {
            format!("{}/{}", self.cwd.trim_end_matches('/'), path)
        }
    }
}

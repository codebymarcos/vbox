use std::sync::Arc;
use tiny_http::{Method, Response, Server};

use crate::scheduler::Scheduler;
use crate::vfs::{Disk, FileSystem};

pub struct HttpDashboard {
    scheduler: Arc<Scheduler>,
    fs: FileSystem,
    disk: Arc<dyn Disk + Send + Sync>,
}

impl HttpDashboard {
    pub fn new(
        scheduler: Arc<Scheduler>,
        fs: FileSystem,
        disk: Arc<dyn Disk + Send + Sync>,
    ) -> Self {
        HttpDashboard {
            scheduler,
            fs,
            disk,
        }
    }

    pub fn start(&self, port: u16) {
        let server = Server::http(format!("127.0.0.1:{}", port)).unwrap();
        println!("HTTP Dashboard started on http://127.0.0.1:{}", port);

        for request in server.incoming_requests() {
            match (request.method(), request.url()) {
                (&Method::Get, "/") => {
                    let response = Response::from_string(self.index_html());
                    request.respond(response).unwrap();
                }
                (&Method::Get, "/api/processes") => {
                    let processes = self.scheduler.list_processes();
                    let json = serde_json::to_string(&processes).unwrap();
                    let response = Response::from_string(json).with_header(
                        tiny_http::Header::from_bytes(
                            &b"Content-Type"[..],
                            &b"application/json"[..],
                        )
                        .unwrap(),
                    );
                    request.respond(response).unwrap();
                }
                (&Method::Get, "/api/memory") => {
                    let memory_info = self.get_memory_info();
                    let json = serde_json::to_string(&memory_info).unwrap();
                    let response = Response::from_string(json).with_header(
                        tiny_http::Header::from_bytes(
                            &b"Content-Type"[..],
                            &b"application/json"[..],
                        )
                        .unwrap(),
                    );
                    request.respond(response).unwrap();
                }
                _ => {
                    let response = Response::from_string("Not Found").with_status_code(404);
                    request.respond(response).unwrap();
                }
            }
        }
    }

    fn index_html(&self) -> String {
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>VBOX Dashboard</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        h1 { color: #333; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
    </style>
    <script>
        async function loadProcesses() {
            const response = await fetch('/api/processes');
            const processes = await response.json();
            const tbody = document.getElementById('processes-tbody');
            tbody.innerHTML = '';
            processes.forEach(p => {
                const row = `<tr>
                    <td>${p.id}</td>
                    <td>${p.priority}</td>
                    <td>${p.status}</td>
                    <td>${p.parent_pid || 'None'}</td>
                    <td>${p.memory_usage} bytes</td>
                </tr>`;
                tbody.innerHTML += row;
            });
        }

        async function loadMemory() {
            const response = await fetch('/api/memory');
            const memory = await response.json();
            document.getElementById('memory-info').innerText = JSON.stringify(memory, null, 2);
        }

        window.onload = () => {
            loadProcesses();
            loadMemory();
        };
    </script>
</head>
<body>
    <h1>VBOX OS Simulator Dashboard</h1>
    <h2>Processes</h2>
    <table>
        <thead>
            <tr>
                <th>PID</th>
                <th>Priority</th>
                <th>Status</th>
                <th>Parent PID</th>
                <th>Memory</th>
            </tr>
        </thead>
        <tbody id="processes-tbody">
        </tbody>
    </table>
    <h2>Memory Info</h2>
    <pre id="memory-info"></pre>
</body>
</html>
        "#
        .to_string()
    }

    fn get_memory_info(&self) -> serde_json::Value {
        // Simple memory info
        serde_json::json!({
            "disk_blocks_allocated": 9, // placeholder
            "total_data_size": 171, // placeholder
            "vfs_directories": 1,
            "vfs_files": 0,
            "total_file_data": 0
        })
    }
}

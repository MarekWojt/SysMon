use sysinfo::{DiskUsage, Process, ProcessExt, System, SystemExt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessDiskUsage {
  total_written_bytes: u64,
  written_bytes: u64,
  total_read_bytes: u64,
  read_bytes: u64,
}

impl From<DiskUsage> for ProcessDiskUsage {
    fn from(origin: DiskUsage) -> Self {
      Self {
        total_written_bytes: origin.total_written_bytes,
        written_bytes: origin.written_bytes,
        total_read_bytes: origin.total_read_bytes,
        read_bytes: origin.total_read_bytes,
      }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
  name: String,
  memory: u64,
  virtual_memory: u64,
  cpu_usage: f32,
  disk_usage: ProcessDiskUsage,
}

impl From<&Process> for ProcessInfo {
    fn from(process: &Process) -> Self {
      Self {
        name: process.name().into(),
        memory: process.memory(),
        virtual_memory: process.virtual_memory(),
        cpu_usage: process.cpu_usage(),
        disk_usage: process.disk_usage().into(),
      }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessesInfo {
  processes: Vec<ProcessInfo>,
}

impl ProcessesInfo {
  pub fn from_system(system: &System) -> Self {
    let raw_processes = system.get_processes();
    let processes = raw_processes.iter().map(|(_, p)| ProcessInfo::from(p)).collect();

    Self {
      processes,
    }
  }
}

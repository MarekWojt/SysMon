use cpu::CpuInfo;
use sysinfo::System;

use self::{mem::MemInfo, processes::ProcessesInfo};

use serde::{Deserialize, Serialize};

pub mod cpu;
pub mod mem;
pub mod temp;
pub mod processes;

#[derive(Debug, Serialize, Deserialize)]
pub struct SysData {
  cpu: CpuInfo,
  mem: MemInfo,
  processes: ProcessesInfo,
}

impl SysData {
  pub fn from_system(system: &mut System) -> Self {
    Self {
      cpu: CpuInfo::from_system(system),
      mem: MemInfo::from_system(system),
      processes: ProcessesInfo::from_system(system),
    }
  }
}

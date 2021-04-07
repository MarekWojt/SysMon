use sysinfo::{System, SystemExt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemHolderInfo {
  total: u64,
  free: u64,
  available: u64,
  used: u64,
}

impl MemHolderInfo {
  pub fn from_swap(system: &System) -> Self {
    Self {
      total: system.get_total_swap(),
      free: system.get_free_swap(),
      available: system.get_free_swap(),
      used: system.get_used_swap(),
    }
  }

  pub fn from_memory(system: &System) -> Self {
    Self {
      total: system.get_total_memory(),
      free: system.get_free_memory(),
      available: system.get_available_memory(),
      used: system.get_used_memory(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemInfo {
  memory: MemHolderInfo,
  swap: MemHolderInfo,
}

impl MemInfo {
  pub fn from_system(system: &System) -> Self {
    Self {
      memory: MemHolderInfo::from_memory(system),
      swap: MemHolderInfo::from_swap(system),
    }
  }
}

use sysinfo::{Processor, ProcessorExt, System, SystemExt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuCoreInfo {
  usage: f32,
  frequency: u64,
}

impl From<&Processor> for CpuCoreInfo {
    fn from(processor: &Processor) -> Self {
      Self {
        usage: processor.get_cpu_usage(),
        frequency: processor.get_frequency(),
      }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
  cores: Vec<CpuCoreInfo>,
  average: f64,
}

impl CpuInfo {
  pub fn from_system(system: &System) -> Self {
    let processors = system.get_processors();
    let cores = processors.iter().map(CpuCoreInfo::from).collect();

    Self {
      average: system.get_load_average().one,
      cores,
    }
  }
}

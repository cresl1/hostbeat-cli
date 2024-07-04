use std::fmt;
use json::{JsonValue, object};

pub struct System {
    pub cpu_cores: Vec<Cpu>,
    pub memory: Memory,
}

pub struct Cpu {
    pub core_id: u8,
    pub usage: f32,
}

pub struct Memory {
    pub total: u64,
    pub used: u64,
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}%", self.core_id, self.usage)
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", (self.used / 1048576), (self.total / 1048576))
    }
}

impl System {

    /// Gets system data like cpu, memory
    /// and returns it in an instance.
    pub fn get_system_data() -> Self {

        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu();

        let memory = {
            let total = sys.total_memory();
            let used = sys.used_memory();
            Memory {
                total,
                used
            }
        };
        
        let mut cpus: Vec<Cpu> = vec![];
        let mut cpu_number: u8 = 1;

        for c in sys.cpus() {
            cpus.push({
                let usage = c.cpu_usage();
                Cpu {
                    core_id: cpu_number,
                    usage
                }
            });
            cpu_number += 1;
        }


        Self {
            cpu_cores: cpus,
            memory
        }
    }

    pub fn to_json(&self, beautify: bool) -> String {

        let mut cpu_core_objects: Vec<JsonValue> = vec![];

        for cpu_core in &self.cpu_cores {

            let value = object! {
                coreId: cpu_core.core_id.clone(),
                currentUsage: cpu_core.usage.clone()
            };

            cpu_core_objects.push(value);
        }

        let data = object!{
            memory: object! {
                total: self.memory.total.clone(),
                currentUsage: self.memory.used.clone(),
            },
            cpuCores: cpu_core_objects
        };

        if beautify {
            return json::stringify_pretty(data, 4);
        }

        return json::stringify(data);
    }

}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_cpu_usage: f32 = self.cpu_cores.iter().map(|x| &x.usage).sum();
        write!(f, "System: Number of CPUs -> {}, CPU Usage -> {}, Memory -> '{}'", self.cpu_cores.len(), (total_cpu_usage / self.cpu_cores.len() as f32), self.memory)
    }
}
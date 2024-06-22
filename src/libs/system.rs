use std::fmt;

pub struct System(pub Vec<Cpu>, pub Memory);
pub struct Cpu(pub u8, pub f32);
pub struct Memory(pub u64, pub u64);

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}%", self.0, self.1)
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", (self.1 / 1048576), (self.0 / 1048576))
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
            Memory(total, used)
        };
        
        let mut cpus: Vec<Cpu> = vec![];
        let mut cpu_number: u8 = 1;

        for c in sys.cpus() {
            cpus.push({
                let usage = c.cpu_usage();
                Cpu(cpu_number, usage)
            });
            cpu_number += 1;
        }

        
        Self(cpus, memory)
    }

}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_cpu_usage: f32 = self.0.iter().map(|x| &x.1).sum();
        write!(f, "System: Number of CPUs -> {}, CPU Usage -> {}, Memory -> '{}'", self.0.len(), (total_cpu_usage / self.0.len() as f32), self.1)
    }
}
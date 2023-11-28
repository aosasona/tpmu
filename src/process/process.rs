pub struct Process {
    pub pid: u32,
    pub name: String,
    pub command: String,
}

pub(crate) trait Manager {
    fn list_processes(&self) -> Vec<Process>;
}

pub fn list_processes() {}

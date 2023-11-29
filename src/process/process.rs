pub struct Process {
    pub pid: u32,
    pub parent_pid: u32,
    pub name: String,
    pub command: String,
    pub port: Option<u16>,
}

pub(crate) trait Manager {
    fn list_processes(&self) -> Result<Vec<Process>, String>;
}

pub fn list_processes() -> Result<Vec<Process>, String> {
    use crate::darwin::DefaultManager;
    let manager = DefaultManager::new();
    manager.list_processes()
}

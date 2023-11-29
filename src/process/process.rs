#[derive(Debug)]
pub struct Process {
    pub uid: Option<u32>,
    pub pid: Option<u32>,
    pub parent_pid: Option<u32>,
    pub command: Option<String>,
    pub port: Option<u16>,
}

pub(crate) trait Manager {
    fn list_processes(&self) -> Result<Vec<Process>, String>;
}

impl Process {
    pub(crate) fn new() -> Self {
        Self {
            uid: None,
            pid: None,
            parent_pid: None,
            command: None,
            port: None,
        }
    }
}

pub fn list_processes() -> Result<Vec<Process>, String> {
    use crate::default::DefaultManager;
    let manager = DefaultManager::new();
    manager.list_processes()
}

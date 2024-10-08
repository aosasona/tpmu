use crate::{field::*, manager::Manager};

#[derive(Debug, Clone)]
pub struct Process {
    /// The user id
    pub uid: Option<u32>,

    /// The process id
    pub pid: Option<u32>,

    /// The parent process id
    pub parent_pid: Option<u32>,

    /// The command that started the process, depending on the opts, this could be the full path or
    /// the executable name alone
    pub command: Option<String>,

    /// Time since the process started in seconds
    pub time_started: Option<usize>,

    // The terminal the process is running on if any
    pub tty: Option<String>,

    /// The port the process is listening on if any
    pub port: Option<u16>,
}

impl Process {
    pub(crate) fn blank_new() -> Self {
        Self {
            uid: None,
            pid: None,
            parent_pid: None,
            command: None,
            tty: None,
            time_started: None,
            port: None,
        }
    }
}

pub struct ListOpts {
    /// The fields to include in the output
    pub fields: Vec<Field>,

    /// Whether to include the port the process is listening on - this may be expensive
    pub with_ports: bool,
}

impl ListOpts {
    pub fn new(fields: Vec<Field>, with_ports: bool) -> Self {
        Self { fields, with_ports }
    }
}

impl Default for ListOpts {
    fn default() -> Self {
        Self {
            fields: vec![
                Field::Pid,
                Field::Ppid,
                Field::Time,
                Field::Comm(Command::ExecutableName),
            ],
            with_ports: false,
        }
    }
}

pub fn list_processes(opts: ListOpts) -> Result<Vec<Process>, String> {
    let manager = Manager::new();
    let output = manager.run_ps(opts.fields)?;
    let processes = manager.parse_ps_output(&output)?;
    // TODO: handle ports

    Ok(processes)
}

pub fn get_pid_by_port(_port: u16) -> Option<u32> {
    None
}

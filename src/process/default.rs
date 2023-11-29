use crate::Manager;
use std::process::Command;

pub(crate) struct DefaultManager {}

enum Field {
    Uid,
    Pid,
    Ppid,
    Comm,
}

impl Field {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "UID" => Some(Self::Uid),
            "PID" => Some(Self::Pid),
            "PPID" => Some(Self::Ppid),
            "COMM" => Some(Self::Comm),
            _ => None,
        }
    }
}

impl Manager for DefaultManager {
    fn list_processes(&self) -> Result<Vec<crate::Process>, String> {
        let output = match Command::new("ps")
            .arg("-A")
            .arg("-o")
            .arg("uid,pid,ppid,comm")
            .arg("-c")
            .arg("-h")
            .output()
        {
            Ok(output) => output,
            Err(e) => return Err(format!("Error running ps: {}", e)),
        };

        let stdout = match String::from_utf8(output.stdout) {
            Ok(stdout) => stdout,
            Err(e) => return Err(format!("Error parsing ps output: {}", e)),
        };

        let mut processes = Vec::new();

        for line in stdout.lines() {
            processes.push(self.to_process(line)?);
        }

        Ok(processes)
    }
}

impl DefaultManager {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Parse a line of output from `ps` into a `Process`.
    fn to_process(&self, line: &str) -> Result<crate::Process, String> {}
}

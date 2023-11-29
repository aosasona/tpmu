use crate::{Manager, Process};
use std::process::Command;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub(crate) enum Field {
    Uid,
    Pid,
    Ppid,
    Comm,
}

impl Into<String> for Field {
    fn into(self) -> String {
        match self {
            Field::Uid => "UID".to_string(),
            Field::Pid => "PID".to_string(),
            Field::Ppid => "PPID".to_string(),
            Field::Comm => "COMM".to_string(),
        }
    }
}

pub(crate) struct Columns {
    _indexes: Vec<(Field, usize)>,
}

impl Columns {
    pub fn find_by_idx(&self, idx: usize) -> Option<Field> {
        for (field, i) in self._indexes.iter() {
            if i.clone() == idx {
                return Some(field.clone());
            }
        }

        None
    }
}

impl From<Vec<(Field, usize)>> for Columns {
    fn from(value: Vec<(Field, usize)>) -> Self {
        Columns { _indexes: value }
    }
}

pub(crate) struct DefaultManager {}

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

        // TODO: collect errors into their own vector and return that instead of failing fast
        let processes: Vec<Process> = Vec::new();
        let lines = stdout.lines().collect::<Vec<&str>>();
        let indexes = self.get_indexes(self.find_header(lines)?)?;

        Ok(processes)
    }
}

impl DefaultManager {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Parse a line of output from `ps` into a `Process`.
    fn parse(&self, cols: Columns, line: &str) -> Result<crate::Process, String> {
        // TODO: we should not be doing this here, this should be done in the caller and on the
        // first line of output
        let mut process = Process::new();

        for (i, col) in line.split_whitespace().enumerate() {
            let field = match cols.find_by_idx(i) {
                Some(field) => field,
                None => continue,
            };

            match field {
                Field::Uid => {
                    let uid = match col.parse::<u32>() {
                        Ok(uid) => uid,
                        Err(e) => return Err(format!("Error parsing UID: {}, value: {}", e, col)),
                    };
                    process.uid = Some(uid);
                }
                Field::Pid => {
                    let pid = match col.parse::<u32>() {
                        Ok(pid) => pid,
                        Err(e) => return Err(format!("Error parsing PID: {}, value: {}", e, col)),
                    };
                    process.pid = Some(pid);
                }
                Field::Ppid => {
                    let ppid = match col.parse::<u32>() {
                        Ok(ppid) => ppid,
                        Err(e) => return Err(format!("Error parsing PPID: {}, value: {}", e, col)),
                    };
                    process.parent_pid = Some(ppid);
                }
                Field::Comm => {
                    process.command = Some(col.to_string());
                }
            }
        }

        Ok(process)
    }

    /// Essentially goes through all the lines and finds the first instance of the header in order
    /// to use that to find the position of all the columns
    fn find_header(&self, lines: Vec<&str>) -> Result<&str, String> {
        Ok("hello")
    }

    /// Get the indexes of the fields we care about from the header line - this is useful for
    /// parsing the rest of the output
    fn get_indexes(&self, header: &str) -> Result<Columns, String> {
        let mut indexes = Vec::new();

        for (i, field) in header.split_whitespace().enumerate() {
            let field = match field {
                "UID" => Field::Uid,
                "PID" => Field::Pid,
                "PPID" => Field::Ppid,
                "COMM" => Field::Comm,
                _ => continue,
            };

            indexes.push((field, i));
        }

        if indexes.is_empty() {
            return Err("No fields found in header".to_string());
        }

        Ok(indexes.into())
    }
}

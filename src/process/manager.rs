use std::process::Command as StdCommand;

use crate::{
    field::Field,
    header::{Header, Parser as HeaderParser},
    Process,
};

pub struct Manager {}

impl Manager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_ps(&self, fields: Vec<Field>) -> Result<String, String> {
        let mut command = StdCommand::new("ps");
        command.arg("-A");

        let mut fields_str_vec = Vec::new();
        for field in self.sort_fields(fields) {
            let field_str: String = field.into();
            if fields_str_vec.contains(&field_str) {
                continue;
            }

            // split the field into the name and the argument if any
            let split: Vec<&str> = field_str.split_whitespace().collect();
            if split.len() > 1 {
                if let Some(name) = split.get(0) {
                    if fields_str_vec.contains(&name.to_string()) {
                        continue;
                    }

                    fields_str_vec.push(name.to_string());
                }

                for arg in split.iter().skip(1) {
                    command.arg(arg);
                }
            } else {
                fields_str_vec.push(field_str);
            }
        }

        // append the fields to the command
        let output = command
            .arg("-o")
            .arg(fields_str_vec.join(","))
            .output()
            .map_err(|e| {
                format!(
                    "Failed to run ps command with fields {:?}: {}",
                    fields_str_vec, e
                )
            })?;

        if !output.status.success() {
            return Err(format!(
                "Failed to run ps command with fields {:?}: {}",
                fields_str_vec,
                String::from_utf8(output.stderr.clone())
                    .map_err(|e| format!("Failed to parse error: {}", e))?
            ));
        }

        let res = String::from_utf8(output.stdout.clone())
            .map_err(|e| format!("Failed to parse output: {}", e))?;

        Ok(res)
    }

    pub fn parse_ps_output(&self, output: &str) -> Result<Vec<Process>, String> {
        let mut lines = output.lines();

        let header = lines
            .next()
            .ok_or_else(|| "No header found in ps output".to_string())?;
        let fields = self.parse_headers(header.into());

        let mut processes = Vec::new();

        for line in lines {
            processes.push(self.parse_line(fields.clone(), line)?);
        }

        Ok(processes)
    }

    fn parse_line(&self, headers: Vec<Header>, line: &str) -> Result<Process, String> {
        let mut process = Process::blank_new();
        for (i, header) in headers.iter().enumerate() {
            // If this is the last item, the end position should be the end of the line itself
            // this is to account for any extra characters
            let end_pos = if i == (headers.len() - 1) {
                line.len()
            } else {
                header.position.end_col
            };

            let raw_value = line[header.position.start_col..end_pos].to_string();
            let value = raw_value.trim();

            match header.field {
                Field::Pid => {
                    process.pid = Some(
                        value
                            .parse::<u32>()
                            .map_err(|e| format!("Failed to parse pid: {}", e))?,
                    )
                }
                Field::Ppid => {
                    process.parent_pid = Some(
                        value
                            .parse::<u32>()
                            .map_err(|e| format!("Failed to parse ppid: {}", e))?,
                    )
                }
                Field::Uid => {
                    process.uid = Some(
                        value
                            .parse::<u32>()
                            .map_err(|e| format!("Failed to parse uid: {}", e))?,
                    )
                }
                Field::Tty => {
                    let tty = value
                        .parse::<String>()
                        .map_err(|e| format!("Failed to parse tty: {}", e))?;
                    if tty.contains("?") {
                        process.tty = None
                    } else {
                        process.tty = Some(tty)
                    }
                }
                Field::Time => {
                    let time = self.parse_time(&value)?;
                    process.time_started = Some(time)
                }
                Field::Comm(_) => {
                    let command = value
                        .parse::<String>()
                        .map_err(|e| format!("Failed to parse command: {} from {}", e, value))?;
                    process.command = Some(command)
                }
            }
        }

        Ok(process)
    }

    fn parse_time(&self, raw_time: &str) -> Result<usize, String> {
        let parts = raw_time.trim().split(":");
        let mut time = 0;

        if let Some(hours) = parts.clone().nth(0) {
            time += hours
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse hours: {}", e))?
                * 60
                * 60;
        }

        if let Some(m_s) = parts.clone().nth(1) {
            let m_s_parts = m_s.split(".");
            if let Some(minutes) = m_s_parts.clone().nth(0) {
                time += minutes
                    .parse::<usize>()
                    .map_err(|e| format!("Failed to parse minutes: {}", e))?
                    * 60;
            }

            if let Some(seconds) = m_s_parts.clone().nth(1) {
                time += seconds
                    .parse::<usize>()
                    .map_err(|e| format!("Failed to parse seconds: {}", e))?;
            }
        }

        Ok(time)
    }

    fn parse_headers(&self, header: String) -> Vec<Header> {
        let mut header_parser = HeaderParser::new(header);
        return header_parser.parse();
    }

    // Sort the fields so that the fields with one identifier are first
    fn sort_fields(&self, mut fields: Vec<Field>) -> Vec<Field> {
        fields.sort_by(|a, _| {
            if a.has_one_ident() {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        });

        return fields;
    }
}

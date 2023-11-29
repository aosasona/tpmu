// 1. Make the command query
// 2. execute the command
// 3. find the headers
// 4. parse the headers to decide what fields to include
// 5. parse the output and skip the headers
// 6. if the position of the field is in the list of fields to include, then include it
//

use std::process::Command as StdCommand;

use crate::field::Field;

pub struct Manager {}

impl Manager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_ps(&self, fields: Vec<Field>) -> Result<String, String> {
        let mut command = StdCommand::new("ps");
        command.arg("-A");

        let mut fields_str_vec = Vec::new();
        for field in fields {
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
}

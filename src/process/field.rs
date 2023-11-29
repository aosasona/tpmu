#[derive(Copy, Clone)]
pub enum Command {
    /// Fetch the full path of the binary that started this process
    FullPath,

    /// Fetch the executable name alone
    ExecutableName,
}

#[derive(Copy, Clone)]
pub enum Field {
    Pid,
    Ppid,
    Uid,
    Tty,
    Time,
    Comm(Command),
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "pid" => Self::Pid,
            "ppid" => Self::Ppid,
            "uid" => Self::Uid,
            "tty" => Self::Tty,
            "time" => Self::Time,
            "comm -c" => Self::Comm(Command::ExecutableName),
            "comm" => Self::Comm(Command::FullPath),
            _ => panic!("Unknown field: {}", value),
        }
    }
}

impl Into<String> for Field {
    fn into(self) -> String {
        match self {
            Self::Pid => "pid".to_string(),
            Self::Ppid => "ppid".to_string(),
            Self::Uid => "uid".to_string(),
            Self::Tty => "tty".to_string(),
            Self::Time => "time".to_string(),
            Self::Comm(command) => match command {
                Command::ExecutableName => "comm -c".to_string(),
                Command::FullPath => "comm".to_string(),
            },
        }
    }
}

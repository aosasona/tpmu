use crate::Manager;

pub(crate) struct DarwinPS {}

impl Manager for DarwinPS {
    fn list_processes(&self) -> Vec<crate::Process> {
        vec![]
    }
}

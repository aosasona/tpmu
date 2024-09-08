use tpmu::{
    field::{Command, Field},
    ListOpts,
};

fn main() {
    let processes = tpmu::list_processes(ListOpts {
        fields: vec![
            Field::Pid,
            Field::Ppid,
            Field::Time,
            Field::Comm(Command::ExecutableName),
        ],
        with_ports: false,
    })
    .expect("Failed to list processes");

    for (count, process) in processes.into_iter().enumerate() {
        if count >= 20 {
            break;
        }
        println!("{:?}-{:?}", process.command, process.time_started);
    }
}

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

    for process in processes {
        println!("{:?}", process.command);
    }
}

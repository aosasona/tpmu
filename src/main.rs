use tpmu::ListProcessesOpts;

fn main() {
    let processes = tpmu::list_processes(ListProcessesOpts::default());
    println!("{:?}", processes);
}

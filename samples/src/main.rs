use tpmu::ListOpts;

fn main() {
    let processes = tpmu::list_processes(ListOpts::default());
    println!("{:?}", processes);
}

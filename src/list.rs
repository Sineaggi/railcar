use oci::State;
use std::io::{self, Write};
use tabwriter::TabWriter;

fn state_printer(states: Vec<State>) {
    let mut tw = TabWriter::new(io::stdout()).padding(3).minwidth(12);
    write!(tw, "ID\tPID\tSTATUS\tBUNDLE\tCREATED\tOWNER\n").unwrap();
    for s in &states {
        write!(tw, "{}\t{}\t{}\t{}\t{}\t{}\n", s.id, s.pid, s.status, s.bundle, "state.created", "state.owner").unwrap();
    }
    tw.flush().unwrap();
}

fn action() {
    //
}

#[cfg(test)]
mod tests {
    use oci::State;
    use std::collections::HashMap;
    use list::state_printer;
    #[test]
    fn it_works() {
        let states = vec![State{
            version: "0.2.0".to_owned(),
            id: "e6fd1c12b16d3f5cf87488be54b2b864265985edbd42b02c318855f2fc99a96d".to_owned(),
            status: "running".to_owned(),
            pid: 4393,
            bundle: "/run/docker/libcontainerd/e6fd1c12b16d3f5cf87488be54b2b864265985edbd42b02c318855f2fc99a96d".to_owned(),
            annotations: HashMap::new(),
        }];
        state_printer(states);
    }
}

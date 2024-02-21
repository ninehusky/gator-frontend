use egglog::EGraph;

// Read a file from command line args and put it in an egraph
fn main() {
    let mut egraph = EGraph::default();
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::read_to_string(filename).unwrap();
    let churchroad_def = std::fs::read_to_string("./churchroad/egglog_src/lakeroad.egg").unwrap();
    let full_program = churchroad_def + "\n" + &file;
    egraph.parse_and_run_program(&full_program).unwrap();
    let map = &egraph.global_bindings;
    for (k, (a, b, c)) in map.iter() {
        println!("{:?}", &egraph.extract_value_to_string(*b));
    }
}

use std::io::Write;

use egglog::EGraph;

// Read a file from command line args and put it in an egraph
fn main() {
    if std::env::args().len() != 3 {
        println!("Usage: cargo run <input_file> <output_file>");
        std::process::exit(1);
    }

    let mut egraph = EGraph::default();
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::read_to_string(filename).unwrap();
    let churchroad_def = "(include \"../churchroad/egglog_src/lakeroad.egg\")".to_string();
    let full_program = churchroad_def + "\n" + &file;
    egraph.parse_and_run_program(&full_program).unwrap();
    let map = &egraph.global_bindings;
    let mut vec = map
        .iter()
        .map(|(_, (_, b, _))| {
            // TODO: there's a way to extract individual let bindings from the egraph
            //       but I don't know how to do it in the context of Churchroad.
            return egraph.extract_value_to_string(*b);
        })
        .collect::<Vec<String>>();
    vec.sort_unstable();
    vec.dedup();

    // make output file which is 2nd command line arg
    let output_file = std::env::args().nth(2).unwrap();
    let mut file = std::fs::File::create(output_file).unwrap();
    for s in &vec {
        file.write(s.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
}

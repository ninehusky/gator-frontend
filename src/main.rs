use egglog::{EGraph, SerializeConfig};

// Read a file from command line args and put it in an egraph
fn main() {
    if std::env::args().len() != 4 {
        println!("Usage: cargo run <input_file> <module_name> <output_file>");
        std::process::exit(1);
    }

    let mut egraph = EGraph::default();
    let filename = std::env::args().nth(1).unwrap();
    // :D
    let module_name = std::env::args().nth(2).unwrap();
    // call yosys on the file
    let yosys_output = String::from_utf8(
        std::process::Command::new("yosys")
            .arg("-q")
            .arg("-p")
            .arg(format!(
                "plugin -i churchroad; read_verilog -sv {}; prep -top {}; pmuxtree; write_lakeroad",
                filename, module_name
            ))
            .output()
            .expect("yosys died")
            .stdout,
    )
    .unwrap();

    let churchroad_def = "(include \"../churchroad/egglog_src/churchroad.egg\")".to_string();
    let full_program = churchroad_def + "\n" + &yosys_output;
    egraph.parse_and_run_program(&full_program).unwrap();
    // run type analysis
    egraph
        .parse_and_run_program("(run-schedule (saturate typing))")
        .unwrap();

    let config = SerializeConfig::default();
    let serialized = egraph.serialize(config);
    // serialized.to_svg_file("tmp.svg");
    let out_filename = std::env::args().nth(3).unwrap();
    serialized.to_json_file(out_filename).unwrap();
}

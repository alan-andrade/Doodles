// Rust experiment to uglify multiple JS using maximum power of
// concurrency.
//

use std::io::fs::walk_dir;
use std::io::Process;
use std::str;

static CORES: uint = 4;

fn main () {
    // Locate the javascript files.
    let js_dir = Path::new("js");

    // Fake developer testing framework
    for path in walk_dir(&js_dir).unwrap() {
        println!("{}", path.display());
    }

    // Ok, now split the job.
    let mut paths = walk_dir(&js_dir).unwrap();
    let size = paths.len();

    // Asume you have 4 cores
    let per_core = size / CORES;

    // Fake developer assurance
    println!("per core: {}", per_core);

    let mut process = match Process::new("uglifyjs", &[~"js/d3.js"]) {
        Ok(p) => p,
        Err(e) => fail!("failed to execute process: {}", e)
    };

    let proc_output = process.wait_with_output();
    print!("output: {:s}", str::from_utf8_owned(proc_output.output).unwrap());
}

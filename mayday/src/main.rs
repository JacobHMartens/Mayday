mod tool;

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    tool::run_tool();

    let execution_time = start.elapsed();
    if execution_time.is_ok() { println!("Total duration: {:?}", execution_time.unwrap()); }
    else { println!("Execution timing failed"); }
}

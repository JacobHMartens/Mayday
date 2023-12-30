mod tool;

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    tool::run_tool();

    let execution_time = start.elapsed();
    println!("Duration: {:?}", execution_time);
}

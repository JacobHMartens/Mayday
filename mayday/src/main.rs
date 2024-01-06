use std::time::SystemTime;
use std::{env, process::Command, path::{PathBuf, Path}, io::{self, Write}};

mod cli;


fn main() {
    let start = SystemTime::now();

    run_tool();

    let execution_time = start.elapsed();
    if execution_time.is_ok() { println!("Total duration: {:?}", execution_time.unwrap()); }
    else { println!("Execution timing failed"); }
}

fn run_tool() {
    let cli_args = cli::get_args();

    // Set current working directory to the target crate's directory
    assert!(env::set_current_dir(cli_args.crate_path).is_ok());

    let compiler_path = ready_compiler();

    // Remove --release from input args such that it can be explicitly added to
    // cargo build (Cargo throws an error if --release is used multiple times).
    let mut cargo_args = cli_args.cargo_build_args;
    cargo_args.retain(|arg| arg != "--release");

    // If the target is already built, then it will not be rebuilt.
    // Running cargo clean removes the build artifacts.
    let cargo_clean_status = Command::new("cargo")
        .args(["clean", "--release"])
        .status();
    println!("Cargo clean success: {:?}", cargo_clean_status);

    // Build the target crate with Cargo using the custom compiler
    let build_target = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .args(cargo_args)
        .env("RUSTC", compiler_path.as_os_str())
        .output()
        .expect("Failed to run tool");
    
    io::stdout().write_all(&build_target.stdout).unwrap();
    io::stderr().write_all(&build_target.stderr).unwrap();
    assert!(build_target.status.success());
}

fn ready_compiler() -> PathBuf {
    match env::current_exe() {
        Ok(tool_path) => return compiler_path_from_tool_path(tool_path),
        Err(e) => panic!("Error getting path of executable. {e}"),
    };
}

fn compiler_path_from_tool_path(tool_path: PathBuf) -> PathBuf {
    let tool_dir = tool_path.parent().unwrap();
    let mut compiler_path = PathBuf::from(tool_dir);
    compiler_path.push("compiler.exe");
    // Build the compiler if the executable doesn't exist
    if !compiler_path.exists() {
        build_compiler(tool_dir);
    }
    // Panic if the compiler executable still doesn't exist
    if !compiler_path.exists() {
        panic!("Unable to locate compiler")
    }
    return compiler_path;
}

fn build_compiler(tool_dir: &Path) {
    // Save current directory (target crate) and change to directory of tool before building compiler
    let crate_dir = env::current_dir().unwrap();
    assert!(env::set_current_dir(tool_dir).is_ok());

    let build_compiler = Command::new("cargo")
        .arg("build")
        .args(["-p", "compiler", "--release"])
        .output()
        .expect("Failed to build compiler");

    io::stderr().write_all(&build_compiler.stderr).unwrap();
    assert!(build_compiler.status.success());

    // Change back to directory of target crate
    assert!(env::set_current_dir(crate_dir).is_ok());
    println!("Compiler built successfully");
}
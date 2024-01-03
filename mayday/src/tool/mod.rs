use std::{env, process::Command, path::PathBuf, io::{self, Write}};

mod cli;

pub fn run_tool() {
    let compiler_path = ready_compiler();
    let args = cli::get_args();
    let build_target = Command::new("cargo")
        .arg("build")
        .args(["-p", &args.target_path, "--release"])
        .arg("--")
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
    let tool_dir = tool_path.parent().expect("Unable to locate directory of executable.");
    let mut compiler_path = PathBuf::from(tool_dir);
    compiler_path.push("compiler.exe");
    // Build the compiler if the executable doesn't exist
    if !compiler_path.exists() {
        build_compiler();
    }
    // Panic if the compiler executable still doesn't exist
    if !compiler_path.exists() {
        panic!("Unable to locate compiler")
    }
    return compiler_path;
}

fn build_compiler() {
    let build_compiler = Command::new("cargo")
        .arg("build")
        .args(["-p", "compiler", "--release"])
        .output()
        .expect("Failed to build compiler");

    io::stderr().write_all(&build_compiler.stderr).unwrap();
    assert!(build_compiler.status.success());
}
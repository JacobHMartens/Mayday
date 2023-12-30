use std::{env, process::Command, path::PathBuf};

mod cli;

pub fn run_tool() {
    let compiler_path = ready_compiler();
    let output = Command::new("cargo")
        .arg("build")
        .args(["-p", "unsafe_example"])
        .arg("--")
        .env("RUSTC", compiler_path.as_os_str())
        .output()
        .expect("Failed to run tool");
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
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
    let build_status = Command::new("cargo")
        .arg("build")
        .args(["-p", "compiler"])
        .status()
        .expect("Failed to build compiler");
    assert!(build_status.success());
}
use std::{fs, process::Command};
use tempfile::tempdir;

/// Gimlet Debugger CLI – launches LLDB with Rust formatters

// This could be good approach but it needs a refactoring on gimlet

// #[derive(Parser)]
// #[command(version, about)]
// struct Args {
//     /// Path to the binary to debug
//     binary: String,
// }

// We need to avoid refactoring big on gimlet before all stuff work


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args = Args::parse();

    // Create a temp dirs to hold script + formatters (exists only while program runs)
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    // Write the shell script
    let run_script_path = temp_path.join("run-lldb.sh");
    fs::write(&run_script_path, include_str!("scripts/run-lldb.sh"))?;

    // Write all required formatter files
    let formatters_dir = temp_path.join("formatters");
    fs::create_dir_all(&formatters_dir)?;

    // At compile time, the `include_str` macro reads the file as a string and ebeds it into the binary, writes it to a file inside the temp_dir
    // I want scripts like (lldb_lookup.py, etc.) to be shipped with the compiled binary and be accessible at runtime by LLDB when script launches
    fs::write(formatters_dir.join("lldb_commands"), include_str!("formatters/lldb_commands"))?;
    fs::write(formatters_dir.join("lldb_lookup.py"), include_str!("formatters/lldb_lookup.py"))?;
    fs::write(formatters_dir.join("lldb_providers.py"), include_str!("formatters/lldb_providers.py"))?;
    fs::write(formatters_dir.join("rust_types.py"), include_str!("formatters/rust_types.py"))?;
    fs::write(formatters_dir.join("solana_commands"), include_str!("formatters/solana_commands"))?;
    fs::write(formatters_dir.join("solana_lookup.py"), include_str!("formatters/solana_lookup.py"))?;
    fs::write(formatters_dir.join("solana_providers.py"), include_str!("formatters/solana_providers.py"))?;
    fs::write(formatters_dir.join("solana_types.py"), include_str!("formatters/solana_types.py"))?;

    // Make script executable
    let _ = Command::new("chmod").arg("+x").arg(&run_script_path).status();

    // Run the script
    let status = Command::new(&run_script_path)
        .current_dir(&temp_path) 
        .status()?;

    if !status.success() {
        eprintln!("--- LLDB launch failed.");
    }

    Ok(())
}

use clap::{Parser, Subcommand};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

#[derive(Parser)]
#[clap(name = "xtask")]
#[clap(about = "Program that help you build and debug d1 flash test project", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build ELF and binary for test flash
    Make {},
    /// View test flash assembly code
    Asm {},
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Make {} => {
            println!("xtask: make D1 flash binary");
            let binutils_prefix = find_binutils_prefix();
            xtask_build_d1_flash_bt0();
            xtask_binary_d1_flash_bt0(binutils_prefix);
        }
        Commands::Asm {  } => {
            println!("xtask: build D1 flash ELF and view assembly");
            let binutils_prefix = find_binutils_prefix();
            xtask_build_d1_flash_bt0();
            xtask_dump_d1_flash_bt0(binutils_prefix);
        }
    }
}

const DEFAULT_TARGET: &'static str = "riscv64imac-unknown-none-elf";

fn xtask_build_d1_flash_bt0() {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command.current_dir(project_root().join("test-d1-flash-bt0"));
    command.arg("build");
    let status = command.status().unwrap();
    if !status.success() {
        eprintln!("xtask: cargo build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_binary_d1_flash_bt0(prefix: &str) {
    let status = Command::new(format!("{}objcopy", prefix))
        .current_dir(dist_dir())
        .arg("test-d1-flash-bt0")
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(&["-O", "binary", "test-d1-flash-bt0.bin"])
        .status()
        .unwrap();

    if !status.success() {
        println!("objcopy binary failed");
        process::exit(1);
    }
}

fn xtask_dump_d1_flash_bt0(prefix: &str) {
    Command::new(format!("{}objdump", prefix))
        .current_dir(dist_dir())
        .arg("test-d1-flash-bt0")
        .arg("-d")
        .status()
        .unwrap();
}

fn find_binutils_prefix() -> &'static str {
    for prefix in ["rust-", "riscv64-unknown-elf-"] {
        let mut command = Command::new(format!("{}objcopy", prefix));
        command.arg("--version");
        command.stdout(Stdio::null());
        let status = command.status().unwrap();
        if status.success() {
            return prefix;
        }
    }
    panic!(
        "error: no binutils found, try install using:
    rustup component add llvm-tools-preview
    cargo install cargo-binutils"
    );
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn dist_dir() -> PathBuf {
    let mut path_buf = project_root().join("target").join(DEFAULT_TARGET);
    // path_buf = match xtask_env.compile_mode {
    //     CompileMode::Debug => path_buf.join("debug"),
    //     CompileMode::Release => path_buf.join("release"),
    // };
    path_buf = path_buf.join("debug");
    path_buf
}

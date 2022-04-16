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
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Make {} => {
            println!("xtask: make d1 flash binary");
            xtask_make_d1_flash_bt0();
        }
    }
}

const DEFAULT_TARGET: &'static str = "riscv64imac-unknown-none-elf";

fn xtask_make_d1_flash_bt0() {
    let objcopy = prepare_objcopy();
    xtask_build_d1_flash_bt0();
    xtask_binary_d1_flash_bt0(objcopy);
}

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

fn xtask_binary_d1_flash_bt0(mut objcopy: Command) {
    let status = objcopy
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

fn prepare_objcopy() -> Command {
    for name in ["rust-objcopy", "riscv64-unknown-elf-objcopy"] {
        let mut command = Command::new(name);
        command.arg("--version");
        command.stdout(Stdio::null());
        let status = command.status().unwrap();
        if status.success() {
            return Command::new(name);
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

mod gdb_detect;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use clap::{Parser, Subcommand};
use std::env;
use std::fs::File;
use std::io::{ErrorKind, Seek, SeekFrom};
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
    /// Make ELF and binary for this project
    Make {
        #[clap(flatten)]
        env: Env,
    },
    /// Build flash and burn into FEL mode board
    Flash {
        #[clap(flatten)]
        env: Env,
    },
    /// View test flash assembly code
    Asm {
        #[clap(flatten)]
        env: Env,
    },
    /// Debug flash code using gdb
    Gdb {
        #[clap(flatten)]
        env: Env,
    },
}

#[derive(clap::Args)]
struct Env {
    #[clap(
        long = "release",
        global = true,
        help = "Build in release mode",
        long_help = None,
    )]
    release: bool,
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Make { env } => {
            println!("xtask: make D1 flash binary");
            let binutils_prefix = find_binutils_prefix_or_fail();
            xtask_build_d1_flash_bt0(env);
            xtask_binary_d1_flash_bt0(binutils_prefix, env);
            xtask_finialize_d1_flash_bt0(env);
        }
        Commands::Flash { env } => {
            println!("xtask: build D1 binary and burn");
            let xfel = find_xfel();
            xfel_find_connected_device(xfel);
            let binutils_prefix = find_binutils_prefix_or_fail();
            xtask_build_d1_flash_bt0(env);
            xtask_binary_d1_flash_bt0(binutils_prefix, env);
            xtask_finialize_d1_flash_bt0(env);
            xtask_burn_d1_flash_bt0(xfel, env);
        }
        Commands::Asm { env } => {
            println!("xtask: build D1 flash ELF and view assembly");
            let binutils_prefix = find_binutils_prefix_or_fail();
            xtask_build_d1_flash_bt0(env);
            xtask_dump_d1_flash_bt0(binutils_prefix, env);
        }
        Commands::Gdb { env } => {
            println!("xtask: debug using gdb");
            xtask_build_d1_flash_bt0(env);
            let gdb_path = if let Ok(ans) = gdb_detect::load_gdb_path_from_file() {
                ans
            } else {
                let ans = gdb_detect::detect_gdb_path();
                gdb_detect::save_gdb_path_to_file(&ans);
                println!("xtask: saved GDB path");
                ans
            };
            let gdb_server = if let Ok(ans) = gdb_detect::load_gdb_server_from_file() {
                ans
            } else {
                let ans = gdb_detect::detect_gdb_server(&gdb_path, env);
                gdb_detect::save_gdb_server_to_file(&ans);
                println!("xtask: saved GDB server");
                ans
            };
            xtask_debug_gdb(&gdb_path, &gdb_server, env);
        }
    }
}

const DEFAULT_TARGET: &'static str = "riscv64imac-unknown-none-elf";

fn xtask_build_d1_flash_bt0(env: &Env) {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command.current_dir(project_root().join("test-d1-flash-bt0"));
    command.arg("build");
    if env.release {
        command.arg("--release");
    }
    let status = command.status().unwrap();
    if !status.success() {
        eprintln!("xtask: cargo build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_binary_d1_flash_bt0(prefix: &str, env: &Env) {
    let status = Command::new(format!("{}objcopy", prefix))
        .current_dir(dist_dir(env))
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

const EGON_HEADER_LENGTH: u64 = 0x60;

// This function does:
// 1. fill in binary length
// 2. calculate checksum of bt0 image; old checksum value must be filled as stamp value
fn xtask_finialize_d1_flash_bt0(env: &Env) {
    let path = dist_dir(env);
    let mut file = File::options()
        .read(true)
        .write(true)
        .open(path.join("test-d1-flash-bt0.bin"))
        .expect("open output binary file");
    let total_length = file.metadata().unwrap().len();
    if total_length < EGON_HEADER_LENGTH {
        panic!(
            "objcopy binary size less than eGON header length, expected >= {} but is {}",
            EGON_HEADER_LENGTH, total_length
        );
    }
    let new_len = align_up_to(total_length, 16 * 1024); // align up to 16KB
    file.set_len(new_len).unwrap();
    file.seek(SeekFrom::Start(0x10)).unwrap();
    file.write_u32::<LittleEndian>(new_len as u32).unwrap();
    file.seek(SeekFrom::Start(0x0C)).unwrap();
    let stamp = file.read_u32::<LittleEndian>().unwrap();
    if stamp != 0x5F0A6C39 {
        panic!("wrong stamp value; check your generated blob and try again")
    }
    let mut checksum: u32 = 0;
    file.seek(SeekFrom::Start(0)).unwrap();
    loop {
        match file.read_u32::<LittleEndian>() {
            Ok(val) => checksum = checksum.wrapping_add(val),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(e) => panic!("io error while calculating checksum: {:?}", e),
        }
    }
    file.seek(SeekFrom::Start(0x0C)).unwrap();
    file.write_u32::<LittleEndian>(checksum).unwrap();
    file.sync_all().unwrap(); // save file before automatic closing
} // for C developers: files are automatically closed when they're out of scope

fn align_up_to(len: u64, target_align: u64) -> u64 {
    let (div, rem) = (len / target_align, len % target_align);
    if rem != 0 {
        (div + 1) * target_align
    } else {
        len
    }
}

fn xtask_burn_d1_flash_bt0(xfel: &str, env: &Env) {
    let mut command = Command::new(xfel);
    command.current_dir(dist_dir(env));
    command.arg("spinand");
    command.args(["write", "0"]);
    command.arg("test-d1-flash-bt0.bin");
    let status = command.status().unwrap();
    if !status.success() {
        println!("objcopy binary failed");
        process::exit(1);
    }
}

fn xtask_dump_d1_flash_bt0(prefix: &str, env: &Env) {
    Command::new(format!("{}objdump", prefix))
        .current_dir(dist_dir(env))
        .arg("test-d1-flash-bt0")
        .arg("-d")
        .status()
        .unwrap();
}

fn xtask_debug_gdb(gdb_path: &str, gdb_server: &str, env: &Env) {
    let mut command = Command::new(gdb_path);
    command.current_dir(dist_dir(env));
    command.args(&["--eval-command", "file test-d1-flash-bt0"]);
    command.args(&["--eval-command", "set architecture riscv:rv64"]);
    command.args(&["--eval-command", "mem 0x0 0xffff ro"]);
    command.args(&["--eval-command", "mem 0x20000 0x27fff rw"]);
    command.args(&["--eval-command", &format!("target remote {}", gdb_server)]);
    command.arg("-q");
    ctrlc::set_handler(move || {
        // when ctrl-c, don't exit gdb
    })
    .expect("disable Ctrl-C exit");
    let status = command.status().unwrap();
    if !status.success() {
        eprintln!("xtask: gdb failed with {}", status);
        process::exit(1);
    }
}

fn find_xfel() -> &'static str {
    let mut command = Command::new("xfel");
    command.stdout(Stdio::null());
    let status = command.status().unwrap();
    if status.success() {
        return "xfel";
    }
    panic!(
        "error: xfel not found
    install xfel from: https://github.com/xboot/xfel"
    );
}

fn xfel_find_connected_device(xfel: &str) {
    let mut command = Command::new(xfel);
    command.arg("version");
    let output = command.output().unwrap();
    if !output.status.success() {
        println!("xfel failed with code {}", output.status);
        println!("Is your device in FEL mode?");
        process::exit(1);
    }
    println!("Found {}", String::from_utf8_lossy(&output.stdout).trim());
}

fn find_binutils_prefix() -> Option<&'static str> {
    for prefix in ["rust-", "riscv64-unknown-elf-", "riscv64-linux-gnu-"] {
        let mut command = Command::new(format!("{}objcopy", prefix));
        command.arg("--version");
        command.stdout(Stdio::null());
        let status = command.status().unwrap();
        if status.success() {
            return Some(prefix);
        }
    }
    None
}

fn find_binutils_prefix_or_fail() -> &'static str {
    if let Some(ans) = find_binutils_prefix() {
        return ans;
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

fn dist_dir(env: &Env) -> PathBuf {
    let mut path_buf = project_root().join("target").join(DEFAULT_TARGET);
    path_buf = match env.release {
        false => path_buf.join("debug"),
        true => path_buf.join("release"),
    };
    path_buf
}

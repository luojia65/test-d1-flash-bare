use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn detect_gdb_path() -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    println!("xtask: you need T-Head toolchain installed to debug this program.");
    loop {
        input.clear();
        print!(
            "Please input T-Head GDB toolchain path:
> "
        );
        stdout.flush().unwrap();
        stdin.read_line(&mut input).expect("read line");
        let mut command = Command::new(&input.trim());
        command.arg("--version");
        let output = match command.output() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("xtask: io error occurred {}", e);
                continue;
            }
        };
        let info = String::from_utf8_lossy(&output.stdout);
        if !info.starts_with("GNU gdb") {
            eprintln!("xtask: not a GNU gdb program");
            continue;
        }
        if info.find("Xuantie-900 elf").is_some() {
            println!("xtask: chosen Xuantie-900 ELF GDB program");
            break;
        } else {
            println!("xtask: chosen generic GDB program");
            break;
        }
    }
    input.trim().to_string()
}

pub fn save_to_file(gdb_path: &str) {
    fs::create_dir_all(project_root().join("target").join("xtask")).expect("create folder");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(
            project_root()
                .join("target")
                .join("xtask")
                .join("gdb-path.txt"),
        )
        .expect("create and open file");
    file.write(gdb_path.as_bytes()).expect("write file");
}

pub fn read_from_file() -> io::Result<String> {
    fs::read_to_string(
        project_root()
            .join("target")
            .join("xtask")
            .join("gdb-path.txt"),
    )
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

use std::path::Path;

fn main() {
    let cwd = std::env::current_dir().unwrap();
    if Path::new("resources/invaders.rom").is_file() {
        println!("cargo:rustc-env=ROM_PATH={}/resources/invaders.rom", cwd.display());
    } else {
        println!("cargo:rustc-env=ROM_PATH={}/resources/dummy.rom", cwd.display());
    }
}
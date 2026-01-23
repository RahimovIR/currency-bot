use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();

    let env_file = Path::new(".env");
    if env_file.exists() {
        let target_env = target_dir.join(".env");
        if let Err(e) = fs::copy(env_file, target_env) {
            eprintln!("Warning: Failed to copy .env file: {}", e);
        }
    }
}

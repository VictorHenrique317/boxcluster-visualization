fn main() {
  #[cfg(target_os="linux")]{
    println!("cargo:rustc-link-lib=python3.10"); 
    // apt install python3.10-dev
    // export PYO3_PYTHON="/usr/bin/python3.10"
    // export LD_LIBRARY_PATH=/usr/lib/python3.10/config-3.10-x86_64-linux-gnu
  }
  tauri_build::build()
}

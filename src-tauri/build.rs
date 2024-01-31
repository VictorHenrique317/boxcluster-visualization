fn main() {
  // #[cfg(target_os="linux")]{
  //   println!("cargo:rustc-link-lib=python3.10"); 
  //   // apt install python3.10-dev
  //   // export PYO3_PYTHON="/usr/bin/python3.10"
  //   // export LD_LIBRARY_PATH=/usr/lib/python3.10/config-3.10-x86_64-linux-gnu
  // }

  // #[cfg(target_os="windows")]{
  //   println!("cargo:rustc-link-lib=python310"); 
  //   // Install python 3.10.11
  //   // set PYO3_PYTHON="C:\\Python310\\python.exe"
  //   // set LIB="C:\\Python310\\libs;%LIB%"
  // }

  tauri_build::build()
}

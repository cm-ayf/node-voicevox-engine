extern crate napi_build;

fn main() {
  #[cfg(target_os = "linux")]
  println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

  #[cfg(any(target_os = "macos", target_os = "ios"))]
  println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/");

  napi_build::setup();
}

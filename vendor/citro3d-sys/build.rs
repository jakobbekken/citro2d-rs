use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use bindgen::{Builder, RustTarget};

fn main() {
    let devkitpro = env::var("DEVKITPRO").expect("DEVKITPRO not set in environment");
    let devkitarm = env::var("DEVKITARM").expect("DEVKITARM not set in environment");
    let debug_symbols = env::var("DEBUG").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-env-changed=DEVKITPRO");
    println!("cargo:rerun-if-env-changed=DEVKITARM");
    println!("cargo:rerun-if-env-changed=DEBUG");
    println!("cargo:rustc-link-search=native={devkitpro}/libctru/lib");
    println!("cargo:rustc-link-lib=static={}", if matches!(debug_symbols.as_str(), "0" | "false" | "none") { "citro3d" } else { "citro3dd" });

    let include_path = Path::new(&devkitpro).join("libctru/include");
    let sysroot = Path::new(&devkitarm).join("arm-none-eabi");
    let system_include = sysroot.join("include");
    let gcc = Path::new(&devkitarm).join("bin/arm-none-eabi-gcc");
    let gcc_include = Path::new(&devkitarm)
        .join("lib/gcc/arm-none-eabi")
        .join(gcc_version(&gcc))
        .join("include");

    let bindings = Builder::default()
        .header(normalize_path(&include_path.join("3ds.h")))
        .header(normalize_path(&include_path.join("citro3d.h")))
        .header(normalize_path(&include_path.join("tex3ds.h")))
        .rust_target(RustTarget::nightly())
        .use_core()
        .trust_clang_mangling(false)
        .layout_tests(false)
        .ctypes_prefix("::libc")
        .prepend_enum_name(false)
        .fit_macro_constants(true)
        .raw_line("use ctru_sys::*;")
        .raw_line("use libc::FILE;")
        .must_use_type("Result")
        .blocklist_type("u(8|16|32|64)")
        .blocklist_type("FILE")
        .opaque_type("(GPU|GFX)_.*")
        .opaque_type("float24Uniform_s")
        .allowlist_file(r".*[\\/]c3d[\\/].*[.]h")
        .allowlist_file(r".*[\\/]tex3ds[.]h")
        .blocklist_file(r".*[\\/]3ds[\\/].*[.]h")
        .blocklist_file(r".*[\\/]sys[\\/].*[.]h")
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("citro3d_statics_wrapper"))
        .clang_args([
            "--target=arm-none-eabi".to_string(),
            "--sysroot".to_string(), normalize_path(&sysroot),
            "-isystem".to_string(), normalize_path(&system_include),
            "-isystem".to_string(), normalize_path(&gcc_include),
            "-I".to_string(), normalize_path(&include_path),
            "-mfloat-abi=hard".to_string(), "-march=armv6k".to_string(),
            "-mtune=mpcore".to_string(), "-mfpu=vfp".to_string(),
            "-DARM11".to_string(), "-D_3DS".to_string(), "-D__3DS__".to_string(),
            "-fshort-enums".to_string(),
        ])
        .generate()
        .expect("unable to generate bindings");

    bindings.write_to_file(out_dir.join("bindings.rs")).expect("failed to write bindings");

    cc::Build::new()
        .compiler(&gcc)
        .archiver(Path::new(&devkitarm).join("bin/arm-none-eabi-ar"))
        .include(&include_path)
        .file(out_dir.join("citro3d_statics_wrapper.c"))
        .flag("-march=armv6k").flag("-mtune=mpcore").flag("-mfloat-abi=hard")
        .flag("-mfpu=vfp").flag("-mtp=soft").flag("-Wno-deprecated-declarations")
        .compile("citro3d_statics_wrapper");
}

fn gcc_version(path: &Path) -> String {
    String::from_utf8_lossy(&Command::new(path).arg("--version").output().unwrap().stdout)
        .split_whitespace().nth(4).unwrap().to_string()
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

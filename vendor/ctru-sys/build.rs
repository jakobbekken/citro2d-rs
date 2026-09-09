use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use bindgen::{Builder, RustTarget};

fn main() {
    let devkitpro = env::var("DEVKITPRO").expect("DEVKITPRO not set in environment");
    let devkitarm = env::var("DEVKITARM").expect("DEVKITARM not set in environment");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DEVKITPRO");
    println!("cargo:rustc-link-search=native={devkitpro}/libctru/lib");
    println!("cargo:rustc-link-lib=static=ctrud");
    println!("cargo:rerun-if-changed={devkitpro}/libctru");

    let bin_dir = Path::new(&devkitarm).join("bin");
    let cc = bin_dir.join("arm-none-eabi-gcc");
    let ar = bin_dir.join("arm-none-eabi-ar");
    let include_path = Path::new(&devkitpro).join("libctru/include");
    let sysroot = Path::new(&devkitarm).join("arm-none-eabi");
    let system_include = sysroot.join("include");
    let gcc_version = gcc_version(&cc);
    let gcc_include = Path::new(&devkitarm)
        .join("lib/gcc/arm-none-eabi")
        .join(gcc_version)
        .join("include");

    let bindings = Builder::default()
        .header(normalize_path(&include_path.join("3ds.h")))
        .header(normalize_path(&system_include.join("errno.h")))
        .rust_target(RustTarget::nightly())
        .use_core()
        .trust_clang_mangling(false)
        .must_use_type("Result")
        .layout_tests(true)
        .ctypes_prefix("::libc")
        .prepend_enum_name(false)
        // Clang reports Windows paths with either separator depending on the tool.
        .allowlist_file(r".*[\\/]3ds[.]h")
        .allowlist_file(r".*[\\/]3ds[\\/].*")
        .allowlist_function("__errno")
        .blocklist_function("gethost(id|name)")
        .blocklist_type("u(8|16|32|64)")
        .blocklist_type("__builtin_va_list")
        .blocklist_type("__va_list")
        .blocklist_type("timeval")
        .blocklist_type("in_addr")
        .blocklist_type("sockaddr_storage")
        .blocklist_type("(in_addr|wchar|socklen|suseconds|sa_family|time)_t")
        .blocklist_item("SOL_CONFIG")
        .derive_default(true)
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("libctru_statics_wrapper"))
        .wrap_unsafe_ops(true)
        .clang_args([
            "--target=arm-none-eabi".to_string(),
            "--sysroot".to_string(),
            normalize_path(&sysroot),
            "-isystem".to_string(),
            normalize_path(&system_include),
            "-isystem".to_string(),
            normalize_path(&gcc_include),
            "-I".to_string(),
            normalize_path(&include_path),
            "-fshort-enums".to_string(),
            "-DARM11".to_string(),
            "-D__3DS__".to_string(),
            "-march=armv6k".to_string(),
            "-mtune=mpcore".to_string(),
            "-mfloat-abi=hard".to_string(),
            "-mfpu=vfp".to_string(),
        ])
        .generate()
        .expect("unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write bindings");

    cc::Build::new()
        .compiler(cc)
        .archiver(ar)
        .include(&include_path)
        .define("ARM11", None)
        .define("__3DS__", None)
        .flag("-march=armv6k")
        .flag("-mtune=mpcore")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=vfp")
        .flag("-mtp=soft")
        .flag("-Wno-deprecated-declarations")
        .file(out_dir.join("libctru_statics_wrapper.c"))
        .compile("ctru_statics_wrapper");
}

fn gcc_version(path: &Path) -> String {
    let output = Command::new(path)
        .arg("--version")
        .output()
        .expect("failed to run arm-none-eabi-gcc");
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .nth(4)
        .expect("unable to determine GCC version")
        .to_string()
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

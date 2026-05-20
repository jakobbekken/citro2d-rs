use std::env;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use bindgen::callbacks::{DeriveTrait, ImplementsTrait, ParseCallbacks};
use bindgen::{Builder, RustTarget};

fn main() {
    let devkitpro = env::var("DEVKITPRO").expect("DEVKITPRO not set in environment");
    println!("cargo:rerun-if-env-changed=DEVKITPRO");

    let devkitarm = env::var("DEVKITARM").expect("DEVKITARM not set in environment");
    println!("cargo:rerun-if-env-changed=DEVKITARM");

    let debug_symbols = env::var("DEBUG").unwrap();
    println!("cargo:rerun-if-env-changed=DEBUG");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let include_path = PathBuf::from_iter([devkitpro.as_str(), "libctru", "include"]);
    let sysroot = Path::new(&devkitarm).join("arm-none-eabi");
    let system_include = sysroot.join("include");
    let static_fns_path = Path::new("citro2d_statics_wrapper");

    let gcc_dir = PathBuf::from_iter([devkitarm.as_str(), "lib", "gcc", "arm-none-eabi"]);
    let gcc_include = gcc_dir
        .read_dir()
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
        .join("include");

    println!("cargo:rustc-link-search=native={devkitpro}/libctru/lib");
    println!(
        "cargo:rustc-link-lib=static={}",
        match debug_symbols.as_str() {
            "0" | "false" | "none" => "citro2d",
            _ => "citro2dd",
        }
    );
    println!(
        "cargo:rustc-link-lib=static={}",
        match debug_symbols.as_str() {
            "0" | "false" | "none" => "citro3d",
            _ => "citro3dd",
        }
    );

    let bindings = Builder::default()
        .header("wrapper.h")
        .rust_target(RustTarget::nightly())
        .use_core()
        .trust_clang_mangling(false)
        .layout_tests(false)
        .ctypes_prefix("::libc")
        .prepend_enum_name(false)
        .fit_macro_constants(true)
        .must_use_type("Result")
        .blocklist_type("u(8|16|32|64)")
        .allowlist_file(".*/c2d/.*[.]h")
        .allowlist_file(".*/citro2d[.]h")
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join(static_fns_path))
        .clang_args([
            "--target=arm-none-eabi",
            "--sysroot",
            sysroot.to_str().unwrap(),
            "-isystem",
            system_include.to_str().unwrap(),
            "-isystem",
            gcc_include.to_str().unwrap(),
            "-I",
            include_path.to_str().unwrap(),
            "-mfloat-abi=hard",
            "-march=armv6k",
            "-mtune=mpcore",
            "-mfpu=vfp",
            "-DARM11",
            "-D_3DS",
            "-D__3DS__",
            "-fshort-enums",
        ])
        .parse_callbacks(Box::new(CustomCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write bindings");

    let cc = Path::new(&devkitarm).join("bin/arm-none-eabi-gcc");
    let ar = Path::new(&devkitarm).join("bin/arm-none-eabi-ar");

    cc::Build::new()
        .compiler(cc)
        .archiver(ar)
        .include(&include_path)
        .include(env::var("CARGO_MANIFEST_DIR").unwrap())
        .file(out_dir.join(static_fns_path.with_extension("c")))
        .flag("-march=armv6k")
        .flag("-mtune=mpcore")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=vfp")
        .flag("-mtp=soft")
        .flag("-Wno-deprecated-declarations")
        .compile("citro2d_statics_wrapper");
}

#[derive(Debug)]
struct CustomCallbacks;

impl ParseCallbacks for CustomCallbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(doxygen_rs::transform(comment))
    }

    fn blocklisted_type_implements_trait(
        &self,
        name: &str,
        derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        if let DeriveTrait::Copy | DeriveTrait::Debug = derive_trait {
            match name {
                "u64_" | "u32_" | "u16_" | "u8_" | "u64" | "u32" | "u16" | "u8" => {
                    Some(ImplementsTrait::Yes)
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

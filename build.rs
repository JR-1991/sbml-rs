//! Build script for SBML Rust bindings
//!
//! This script handles:
//! 1. Building the required C++ libraries (libxml2 and libSBML) from source
//! 2. Generating Rust bindings to the C++ code using autocxx
//! 3. Configuring the build environment and linking
//!
//! The script requires CMake to be installed on the system for building the C++ libraries.

const LIBSBML_NAME: &str = "sbml";
const LIBSBML_PATH: &str = "vendors/libsbml";
const LIBSBML_DEPENDENCY_DIR: &str = "vendors/libsbml-dependencies";

const WITH_LIBXML: &str = "OFF";
const WITH_EXPAT: &str = "True";
const WITH_STATIC_RUNTIME: &str = if cfg!(target_os = "windows") {
    "ON"
} else {
    "OFF"
};

fn main() -> miette::Result<()> {
    // Ensure cargo rebuilds if this build script changes
    println!("cargo:rerun-if-changed=build.rs");

    // Build and link libSBML
    let sbml_build = build_and_link(LIBSBML_PATH, LIBSBML_NAME, false)?;

    // Configure autocxx to generate Rust bindings
    let rs_file = "src/lib.rs";

    // Point to the libSBML headers
    let sbml_include = format!("{}/include", sbml_build);
    let lib_root = ".";

    // Build the C++ wrapper code and bindings
    let mut b = autocxx_build::Builder::new(rs_file, &[lib_root, &sbml_include]).build()?;

    // Ensure C++17 is used for compilation
    b.flag_if_supported("-std=c++17").compile("libsbml");

    Ok(())
}

/// Helper function to build and link a C++ library using CMake
///
/// # Arguments
/// * `path` - Path to the library source directory
/// * `lib_name` - Name of the library to link against
/// * `static_lib` - Whether to link against a static library
///
/// # Returns
/// * The build directory path as a String
fn build_and_link(path: &str, lib_name: &str, static_lib: bool) -> miette::Result<String> {
    let dst = if cfg!(target_os = "windows") {
        cmake::Config::new(path)
            .define("WITH_STATIC_RUNTIME", WITH_STATIC_RUNTIME)
            .define("WITH_LIBXML", WITH_LIBXML)
            .define("WITH_EXPAT", WITH_EXPAT)
            .define("LIBSBML_DEPENDENCY_DIR", LIBSBML_DEPENDENCY_DIR)
            .build()
    } else {
        cmake::Config::new(path)
            .define("WITH_STATIC_RUNTIME", WITH_STATIC_RUNTIME)
            .define("WITH_LIBXML", WITH_LIBXML)
            .define("WITH_EXPAT", WITH_EXPAT)
            .build()
    };

    // Configure cargo to link against the built library
    println!("cargo:rustc-link-search={}/lib", dst.display());

    if static_lib {
        println!("cargo:rustc-link-lib=static={}", lib_name);
    } else {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }

    Ok(dst.display().to_string())
}

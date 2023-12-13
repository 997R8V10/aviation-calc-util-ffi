use std::fs::create_dir_all;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_dir = target_dir();

    let nuget_package_name = env::var("NUGET_PACKAGE_NAME").unwrap();

    // Create dir
    create_dir_all(&target_dir.join("build")).unwrap();

    // Load NuGet info and replace variables
    replace_file_vars(
        &crate_dir.join("nugetinfo").join("template.nuspec"),
        &target_dir.join(format!("{}.nuspec", nuget_package_name)),
    );
    replace_file_vars(
        &crate_dir.join("nugetinfo").join("template.targets"),
        &target_dir.join("build").join(format!("{}.targets", nuget_package_name)),
    );    
    replace_file_vars(
        &crate_dir.join("README.md"),
        &target_dir.join("README.md"),
    );    
    replace_file_vars(
        &crate_dir.join("LICENSE"),
        &target_dir.join("LICENSE.txt"),
    );
}

fn replace_file_vars(in_file: &PathBuf, out_file: &PathBuf) {
    let mut contents = fs::read_to_string(in_file).unwrap();

    let rules = &[
        ("${NUGET_PACKAGE_NAME}", env::var("NUGET_PACKAGE_NAME").unwrap()),
        ("${CARGO_PKG_VERSION}", env::var("CARGO_PKG_VERSION").unwrap()),
        ("${CARGO_PKG_AUTHORS}", env::var("CARGO_PKG_AUTHORS").unwrap()),
        ("${CARGO_PKG_README}", env::var("CARGO_PKG_README").unwrap()),
        ("${CARGO_PKG_DESCRIPTION}", env::var("CARGO_PKG_DESCRIPTION").unwrap()),
        ("${CARGO_PKG_HOMEPAGE}", env::var("CARGO_PKG_HOMEPAGE").unwrap()),
        ("${CARGO_PKG_REPOSITORY}", env::var("CARGO_PKG_REPOSITORY").unwrap()),
    ];

    for (from, to) in rules {
        while let Some(start) = contents.find(from) {
            let range = start..start + from.len();
            contents.replace_range(range, to);
        }        
    }

    fs::write(out_file, contents).unwrap();
}

fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("NUGET_DEFS_OUTPUT") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}

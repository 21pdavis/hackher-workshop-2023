use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("build\\msvc");
            dll_dir.push("build\\msvc");
        }
        else {
            lib_dir.push("build\\gnu-mingw");
            dll_dir.push("build\\gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        }
        else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        let lib_dir_disp = lib_dir.display();
        // panic!("We are here and lib_dir_disp is {}", lib_dir_disp);
        println!("cargo:rustc-link-search=all={}", lib_dir_disp);
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir")  {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }

    #[cfg(target_os="macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");

    #[cfg(target_os="linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    
}

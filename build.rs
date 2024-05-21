
use std::env;
use std::path::PathBuf;

fn main() {
    
    // let lib_path = PathBuf::from(env::current_exe().unwrap()).join("");
    // println!("cargo:rustc-link-search=native={}", lib_path.display());

    // 链接C++标准库
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=c++abi");
    // 告诉Cargo静态库的位置
    let lib_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("");
    println!("cargo:warning=zys: {}",lib_path.display() );
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    // 告诉Cargo链接静态库
    println!("cargo:rustc-link-lib=dylib=RecastNavLib");


    prost_build::Config::new()
        .out_dir("src/pb") // proto输出目录
        .compile_protos(
            &[
                "proto/role.proto",
                "proto/hello.proto",
                "proto/login.proto",
            ]
            , &["."]) // 要处理的proto文件
        .expect("编译proto失败");
    
}
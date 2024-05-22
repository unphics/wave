
use std::env;
use std::path::PathBuf;

const PREPARE_ALL_LIB_FORCE: bool = false; // 强制重新准备所有库

fn main() {
    println!("cargo:rustc-link-search=native=target/debug/");
    println!("cargo:rustc-link-lib=dylib=RecastNav");
    let recast_nav_src_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("libs/recast_nav/build/libRecastNav.so");
    let recast_nav_dst_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target/debug/libRecastNav.so");
    if PREPARE_ALL_LIB_FORCE || !recast_nav_dst_path.exists() {
        std::fs::copy(&recast_nav_src_path, &recast_nav_dst_path).expect("");
    }

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
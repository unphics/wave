fn main() {
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
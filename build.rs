fn main() {
    prost_build::Config::new()
        .out_dir("src/pb") // proto输出目录
        .compile_protos(
            &[
                "proto/hello.proto"
            ]
            , &["."]) // 要处理的proto文件
        .unwrap();
}
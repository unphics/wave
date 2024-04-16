// build.rs可以在编译cargo项目时做额外的编译处理
fn main() {
    prost_build::Config::new()
        .out_dir("scr/pb")
        .complie_protos(&["abi.proto"], &["."])
        .unwrap();
}
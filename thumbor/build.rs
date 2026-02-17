use prost_build::Config;

fn main() {
    Config::new()
        .out_dir("src/pb") // 要先创建对应的目录
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}

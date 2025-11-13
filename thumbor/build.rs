use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("src/pb") //输出目录
        .compile_protos(&["src/abi.proto"], &["."])?;

    Ok(())
}

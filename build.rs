fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files: Vec<_> = glob::glob("src/proto/**/*.proto")?
        .filter_map(Result::ok)
        .collect();

    prost_build::Config::new()
        .compile_protos(&proto_files, &["src/proto"])?;

    Ok(())
}
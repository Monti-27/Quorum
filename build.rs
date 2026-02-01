// build script to compile protobuf definitions

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // compiles proto/custodian.proto into rust types
    tonic_build::compile_protos("proto/custodian.proto")?;
    Ok(())
}

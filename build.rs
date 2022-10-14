fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
                &["proto/helloworld/helloworld.proto"],
                &["proto"]
        )
        .unwrap();
    Ok(())
}
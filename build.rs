fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
                &["proto/crawl/crawl.proto"],
                &["proto"]
        )
        .unwrap();
    Ok(())
}
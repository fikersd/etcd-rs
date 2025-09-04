fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(
            &[
                "proto/auth.proto",
                "proto/kv.proto",
                "proto/rpc.proto",
                "proto/v3lock.proto",
                "proto/v3election.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}

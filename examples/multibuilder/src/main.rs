use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() {
    let result = SpirvBuilder::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/repro"),
        "spirv-unknown-vulkan1.1spv1.4",
    )
    .print_metadata(MetadataPrintout::DependencyOnly)
    .multimodule(true)
    .build()
    .unwrap();
    println!("{:#?}", result);
}

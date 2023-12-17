use std::path::PathBuf;

fn main() {
    let data = schema::schema();
    cortex_generation::CortexGenerator::new(PathBuf::from("src/schema.rs"))
        .create_file(data.1)
        .unwrap();
}

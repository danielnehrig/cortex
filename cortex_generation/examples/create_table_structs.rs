use cortex::{objects::table::TableAnnotation, prelude::*};
use cortex_generation::CortexGenerator;

fn main() {
    let mut table = Table::new("test");
    table = table.add_prop(("id", PropType::Int32, None));
    table = table.add_prop(("name", PropType::Text, None));
    table = table.add_prop(("age", PropType::Int32, None));
    table = table.add_annotation(TableAnnotation::Partition);
    let step = Step::new("test", StepType::Update, semver::Version::new(1, 0, 0))
        .add_statement(table, DbAction::Create);
    CortexGenerator::new(std::path::PathBuf::from("examples/generated.rs"))
        .create_file(vec![step])
        .unwrap();
}

use std::fmt::Display;

use crate::{objects::step::Step, producer::PostgresStatementProducer};

impl Display for Step<'_, PostgresStatementProducer<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = &self
            .statements
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}\n{}", self.name, statements)
    }
}

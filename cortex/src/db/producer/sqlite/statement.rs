use std::fmt::Display;

use crate::{objects::statement::Statement, producer::SQLiteStatementProducer};

impl Display for Statement<'_, SQLiteStatementProducer<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Create(x) => write!(f, "CREATE {}", x.create()),
            Statement::Drop(x) => write!(f, "DROP {}", x.drop()),
            Statement::Alter(_x) => todo!(),
            Statement::Insert(_x) => todo!(),
            Statement::_Phantom(_) => panic!(),
        }
    }
}

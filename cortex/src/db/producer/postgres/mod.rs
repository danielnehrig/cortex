use crate::{
    objects::{
        database::Database,
        procedure::StoredProcedure,
        statement::{DbAction, Statement},
        table::{PropAnnotation, PropType, Table, TableAnnotation, TableProp},
        views::View,
    },
    prelude::{Role, Sequence, User},
};

pub(crate) struct PostgresStatementProducer;

pub(crate) fn table_annotation_to_db(annotation: &PropAnnotation) -> String {
    match annotation {
        PropAnnotation::PrimaryKey => "PRIMARY KEY".to_string(),
        PropAnnotation::Unique => "UNIQUE".to_string(),
        PropAnnotation::NotNull => "NOT NULL".to_string(),
        PropAnnotation::Default => "DEFAULT".to_string(),
        PropAnnotation::Check => "CHECK".to_string(),
        PropAnnotation::Foreign => "FOREIGN".to_string(),
        PropAnnotation::Constraint(_) => "CONSTRAINT".to_string(),
        PropAnnotation::Empty => "".to_string(),
    }
}

pub(crate) fn prop_type_to_db(prop_type: &PropType) -> String {
    match prop_type {
        PropType::Int => "INT".to_string(),
        PropType::Text => "TEXT".to_string(),
        PropType::Bool => "BOOL".to_string(),
        PropType::Date => "DATE".to_string(),
        PropType::Timestamp => "TIMESTAMP".to_string(),
        PropType::BigInt => "BIGINT".to_string(),
        PropType::Double => "DOUBLE".to_string(),
        PropType::SmallInt => "SMALLINT".to_string(),
    }
}
pub(crate) fn compose_prop(prop: &TableProp) -> String {
    let t = prop_type_to_db(&prop.t_type);
    match &prop.annotation.clone() {
        Some(p) => {
            let a = table_annotation_to_db(p);
            format!("{} {} {}", prop.name, t, a)
        }
        None => {
            format!("{} {}", prop.name, t)
        }
    }
}
pub(crate) fn serialize_annotation(annotations: &TableAnnotation) -> String {
    match annotations {
        TableAnnotation::Partition => "PARTITION".to_string(),
        TableAnnotation::View => "VIEW".to_string(),
    }
}

impl PostgresStatementProducer {
    pub fn map(statement: &Statement, action: &DbAction) -> String {
        match statement {
            Statement::Table(t) => PostgresStatementProducer::table_statement(t, action),
            Statement::Database(d) => PostgresStatementProducer::database_statement(d, action),
            Statement::View(v) => PostgresStatementProducer::view_statement(v, action),
            Statement::User(u) => PostgresStatementProducer::user_statement(u, action),
            Statement::Role(r) => PostgresStatementProducer::role_statement(r, action),
            Statement::Sequence(s) => PostgresStatementProducer::sequence_statement(s, action),
        }
    }

    fn stored_procedure(stored_proc: &StoredProcedure, action: &DbAction) -> String {
        match action {
            DbAction::Create => {
                let params = stored_proc
                    .params
                    .iter()
                    .map(|e| format!("{} {}", e.name, e.data_type))
                    .collect::<Vec<String>>()
                    .join(", ");
                let returns = format!(
                    "RETURNS {}",
                    stored_proc.returns.as_ref().unwrap().data_type
                );
                let language = "LANGUAGE SQL".to_string();
                let body = format!("AS $$ {} $$", stored_proc.body);
                format!(
                    "CREATE FUNCTION {} ({}) {} {} {};",
                    stored_proc.name, params, returns, language, body
                )
            }
            DbAction::Drop => format!("DROP FUNCTION IF EXISTS {};", stored_proc.name),
            DbAction::Alter => panic!("altering a stored procedure is not supported"),
            DbAction::Insert => panic!("inserting a stored procedure is not supported"),
            _ => panic!("granting and revoking a stored procedure is not supported"),
        }
    }

    fn sequence_statement(sequence: &Sequence, action: &DbAction) -> String {
        match action {
            DbAction::Create => {
                let start = match sequence.start {
                    Some(s) => format!("START WITH {s}"),
                    None => "".to_string(),
                };
                let increment = match sequence.increment {
                    Some(i) => format!("INCREMENT BY {i}"),
                    None => "".to_string(),
                };
                let min_value = match sequence.min_value {
                    Some(m) => format!("MINVALUE {m}"),
                    None => "".to_string(),
                };
                let max_value = match sequence.max_value {
                    Some(m) => format!("MAXVALUE {m}"),
                    None => "".to_string(),
                };
                let cache = match sequence.cache {
                    Some(c) => format!("CACHE {c}"),
                    None => "".to_string(),
                };
                let cycle = match sequence.cycle {
                    Some(c) => format!("CYCLE {c}"),
                    None => "".to_string(),
                };
                format!(
                    "CREATE SEQUENCE {} {} {} {} {} {} {};",
                    sequence.name, start, increment, min_value, max_value, cache, cycle
                )
            }
            DbAction::Drop => format!("DROP SEQUENCE IF EXISTS {};", sequence.name),
            DbAction::Alter => panic!("altering a sequence is not supported"),
            DbAction::Insert => panic!("inserting a sequence is not supported"),
            _ => panic!("granting and revoking a sequence is not supported"),
        }
    }

    fn role_statement(role: &Role, action: &DbAction) -> String {
        match action {
            DbAction::Create => {
                format!("CREATE ROLE {};", role.name)
            }
            DbAction::Drop => format!("DROP ROLE IF EXISTS {};", role.name),
            DbAction::Alter => panic!("altering a role is not supported"),
            DbAction::Insert => panic!("inserting a role is not supported"),
            DbAction::Grant => {
                todo!()
            }
            DbAction::Revoke => todo!(),
        }
    }

    fn view_statement(view: &View, action: &DbAction) -> String {
        match action {
            DbAction::Create => {
                let props = view
                    .props
                    .iter()
                    .map(|e| e.name.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let from = view.from.join(", ");
                let where_clause = view.where_clause.join(" AND ");
                format!(
                    "CREATE VIEW {} ({}) AS SELECT {} FROM {} WHERE {};",
                    view.name, props, props, from, where_clause
                )
            }
            DbAction::Drop => format!("DROP VIEW IF EXISTS {};", view.name),
            DbAction::Alter => panic!("altering a view is not supported"),
            DbAction::Insert => panic!("inserting a view is not supported"),
            _ => panic!("granting and revoking a view is not supported"),
        }
    }

    fn user_statement(user: &User, action: &DbAction) -> String {
        match action {
            DbAction::Create => {
                let password = match &user.password {
                    Some(p) => format!("PASSWORD '{p}'"),
                    None => "".to_string(),
                };
                format!("CREATE USER {} {password};", user.name)
            }
            DbAction::Drop => format!("DROP USER IF EXISTS {};", user.name),
            DbAction::Alter => panic!("altering a user is not supported"),
            DbAction::Insert => panic!("inserting a user is not supported"),
            _ => panic!("granting and revoking a view is not supported"),
        }
    }

    fn table_statement(table: &Table, action: &DbAction) -> String {
        match action {
            DbAction::Create => {
                let props = table
                    .props
                    .iter()
                    .map(compose_prop)
                    .collect::<Vec<String>>()
                    .join(", ");
                let annotations = table
                    .annotations
                    .iter()
                    .map(serialize_annotation)
                    .collect::<Vec<String>>()
                    .join(" ");

                match (props.is_empty(), annotations.is_empty()) {
                    (true, true) => format!("CREATE TABLE {};", table.name),
                    (true, false) => format!("CREATE TABLE {} {};", table.name, annotations),
                    (false, true) => format!("CREATE TABLE {} ({});", table.name, props),
                    (false, false) => {
                        format!("CREATE TABLE {} ({}) {};", table.name, props, annotations)
                    }
                }
            }
            DbAction::Drop => format!("DROP TABLE IF EXISTS {};", table.name),
            DbAction::Alter => panic!("altering a table is not supported"),
            DbAction::Insert => panic!("inserting a table is not supported"),
            _ => panic!("granting and revoking a view is not supported"),
        }
    }

    fn database_statement(database: &Database, action: &DbAction) -> String {
        match action {
            DbAction::Create => format!("CREATE DATABASE {};", database.name),
            DbAction::Drop => format!("DROP DATABASE {};", database.name),
            DbAction::Alter => panic!("altering a database is not supported"),
            DbAction::Insert => panic!("inserting a database is not supported"),
            _ => panic!("granting and revoking a view is not supported"),
        }
    }
}

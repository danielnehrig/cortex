use std::rc::Rc;

use crate::objects::statement::Statement;

#[derive(Debug, Clone)]
/// Table struxt for creating tables
pub struct Table<T> {
    /// name of the table
    pub name: Rc<str>,
    /// properties of the table
    pub props: Vec<TableProp<T>>,
    /// annotations of the table
    pub annotations: Vec<TableAnnotation>,
    _marker: std::marker::PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct TableProp<T> {
    pub name: Rc<str>,
    pub t_type: PropType,
    pub annotation: Option<PropAnnotation>,
    _marker: std::marker::PhantomData<T>,
}

#[derive(Debug, Clone)]
pub enum PropType {
    Int,
    Double,
    Text,
    Bool,
    Date,
    Timestamp,
    Bigint,
    Smallint,
    // ...
}

#[derive(Debug, Clone, Default)]
pub enum PropAnnotation {
    PrimaryKey,
    Unique,
    NotNull,
    Default,
    Check,
    Foreign,
    Constraint(Box<PropAnnotation>),
    #[default]
    Empty,
}

#[derive(Debug, Clone)]
pub enum TableAnnotation {
    Partition,
    View,
}

impl<T> Table<T> {
    pub fn new(name: &str) -> Self {
        Table {
            name: Rc::from(name),
            props: Vec::new(),
            annotations: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn add_prop(mut self, prop: TableProp<T>) -> Self {
        self.props.push(prop);
        self
    }

    pub fn add_props_as_slice(
        mut self,
        props: &[(&str, PropType, Option<PropAnnotation>)],
    ) -> Self {
        for (name, t_type, annotation) in props {
            self.props
                .push(TableProp::new(name, t_type.clone(), annotation.clone()));
        }
        self
    }

    pub fn add_annotation(mut self, annotation: TableAnnotation) -> Self {
        self.annotations.push(annotation);
        self
    }
}

impl<T> TableProp<T> {
    pub fn new(name: &str, t_type: PropType, annotation: Option<PropAnnotation>) -> Self {
        TableProp {
            name: Rc::from(name),
            t_type,
            annotation,
            _marker: std::marker::PhantomData,
        }
    }
}

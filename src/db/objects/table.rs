use crate::db::producer::DatabaseSpeicifics;

#[derive(Debug, Clone)]
pub struct Table<T: DatabaseSpeicifics + Clone> {
    pub name: &'static str,
    pub props: Vec<TableProp>,
    pub annotations: Vec<TableAnnotation>,
    _marker: std::marker::PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct TableProp {
    pub name: &'static str,
    pub t_type: PropType,
    pub annotation: Option<PropAnnotation>,
}

#[derive(Debug, Clone)]
pub enum TableType {
    Table,
    View,
    MaterializedView,
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
}

#[derive(Debug, Clone, Default)]
pub enum PropAnnotation {
    PrimaryKey,
    Unique,
    NotNull,
    Default,
    Check,
    Foreign,
    #[default]
    Empty,
}

#[derive(Debug, Clone)]
pub enum TableAnnotation {
    Partition,
    View,
}

impl<T: DatabaseSpeicifics + Clone> Table<T> {
    pub fn new(name: &'static str) -> Self {
        Table {
            name,
            props: Vec::new(),
            annotations: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn add_prop(&mut self, prop: TableProp) -> Self {
        self.props.push(prop);
        self.to_owned()
    }

    pub fn add_annotation(&mut self, annotation: TableAnnotation) -> Self {
        self.annotations.push(annotation);
        self.to_owned()
    }
}

impl TableProp {
    pub fn new(name: &'static str, t_type: PropType, annotation: Option<PropAnnotation>) -> Self {
        TableProp {
            name,
            t_type,
            annotation,
        }
    }
}

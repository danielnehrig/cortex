use std::rc::Rc;

use crate::objects::statement::Statement;

#[doc(alias = "Collection")]
#[derive(Debug, Clone)]
/// Table struct for creating tables
pub struct Table {
    /// name of the table
    pub name: Rc<str>,
    /// properties of the table
    /// # Disclaimer
    /// - MongoDB  : does support collection creation without properties since its nosql
    ///              and a collection could have entries with different properties
    ///              while this is possible it is not recommended since we create schemas with this
    ///              lib
    /// - Postgres : Requires at least one property to be created
    pub props: Vec<TableProp>,
    /// annotations of the table
    pub annotations: Vec<TableAnnotation>,
    /// database of the table
    pub database: Option<Rc<str>>,
    /// namespace
    pub namespace: Option<Rc<str>>,
}

#[derive(Debug, Clone)]
/// TableProp struct for creating properties of a table
pub struct TableProp {
    pub field: TableField,
    pub field_type: PropType,
    pub annotation: Option<PropAnnotation>,
}

#[derive(Debug, Clone)]
pub enum TableField {
    Text(Rc<str>),
    Annotation(TableAnnotation),
}

impl TableField {
    pub fn get_text(&self) -> String {
        match self {
            TableField::Text(text) => text.to_string(),
            _ => panic!("TableField is not a text field"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// PropType enum for defining the type of a property
pub enum PropType {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Double,
    Text,
    Bool,
    Date,
    Timestamp,
    BigInt,
    SmallInt,
    // ...
}

impl PropType {
    pub fn to_rust_type(&self) -> String {
        match self {
            PropType::Int8 => "i8".into(),
            PropType::Int16 => "i16".into(),
            PropType::Int32 => "i32".into(),
            PropType::Int64 => "i64".into(),
            PropType::UInt8 => "u8".into(),
            PropType::UInt16 => "u16".into(),
            PropType::UInt32 => "u32".into(),
            PropType::UInt64 => "u64".into(),
            PropType::Double => "f64".into(),
            PropType::Text => "String".into(),
            PropType::Bool => "bool".into(),
            PropType::Date => "chrono::NaiveDate".into(),
            PropType::Timestamp => "chrono::NaiveDateTime".into(),
            PropType::BigInt => "i64".into(),
            PropType::SmallInt => "i16".into(),
        }
    }
}

#[derive(Debug, Clone)]
/// PropAnnotation enum for defining the annotation of a property
pub enum PropAnnotation {
    PrimaryKey,
    Unique,
    NotNull,
    Default,
    Check,
    Identity,
    ForeignKey(Table),
    Constraint(Box<PropAnnotation>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// TableAnnotation enum for defining the annotation of a table
pub enum TableAnnotation {
    Partition,
    View,
}

impl Table {
    /// Create a new table
    /// # Example
    /// ```
    /// use cortex::objects::table::{Table};
    /// let table  = Table::new("table");
    /// assert_eq!(table.name, "table".into());
    /// ```
    pub fn new(name: &str) -> Self {
        Table {
            name: Rc::from(name),
            namespace: None,
            props: Vec::new(),
            annotations: Vec::new(),
            database: None,
        }
    }

    /// Add a property to the table
    /// # Example
    /// ```
    /// use cortex::objects::table::{Table, TableProp, PropType};
    /// let table = Table::new("table")
    ///    .add_prop(("id", PropType::Int32, None))
    ///    .add_prop(("name", PropType::Text, None))
    ///    .add_prop(("age", PropType::Int32, None));
    ///  assert_eq!(table.props.len(), 3);
    ///  assert_eq!(table.props[0].name, "id".into());
    ///  assert_eq!(table.props[1].name, "name".into());
    ///  assert_eq!(table.props[2].name, "age".into());
    /// ```
    pub fn add_prop(
        mut self,
        (name, prop_type, annotation): (&str, PropType, Option<PropAnnotation>),
    ) -> Self {
        self.props.push(TableProp::new(name, prop_type, annotation));
        self
    }

    /// Attach a database to the table context
    /// This allows for database specific operations
    /// Some databases may not support this others might
    /// # Disclaimer
    /// - MongoDB  : does not create databases like SQL databases
    ///              to create a collection you need to specify a database
    ///
    /// - Postgres : does not support this the table will be executed on the associated connection
    ///              to the database
    /// # Example
    /// ```
    /// use cortex::objects::table::{Table};
    /// let table = Table::new("table")
    ///   .on_db("db");
    ///   assert_eq!(table.database.unwrap(), "db".into());
    /// ```
    pub fn on_db(mut self, db: impl Into<Rc<str>>) -> Self {
        self.database = Some(db.into());
        self
    }

    /// Add a properties to the table
    /// # Example
    /// ```
    /// use cortex::objects::table::{Table, TableProp, PropType};
    /// let table = Table::new("table")
    ///    .add_props_as_slice(&[
    ///    ("id", PropType::Int32, None),
    ///    ("name", PropType::Text, None),
    ///    ("age", PropType::Int32, None),
    ///    ]);
    ///  assert_eq!(table.props.len(), 3);
    ///  assert_eq!(table.props[0].name, "id".into());
    ///  assert_eq!(table.props[1].name, "name".into());
    ///  assert_eq!(table.props[2].name, "age".into());
    /// ```
    pub fn add_props_as_slice(
        mut self,
        props: &[(&str, PropType, Option<PropAnnotation>)],
    ) -> Self {
        for (name, t_type, annotation) in props.iter() {
            self.props
                .push(TableProp::new(*name, *t_type, annotation.clone()));
        }
        self
    }

    /// Add an annotation to the table
    /// # Example
    /// ```
    /// use cortex::objects::table::{Table, TableProp, PropType, TableAnnotation};
    /// let table = Table::new("table")
    ///    .add_prop(("id", PropType::Int32, None))
    ///    .add_prop(("name", PropType::Text, None))
    ///    .add_prop(("age", PropType::Int32, None))
    ///    .add_annotation(TableAnnotation::Partition);
    ///  assert_eq!(table.props.len(), 3);
    ///  assert_eq!(table.props[0].name, "id".into());
    ///  assert_eq!(table.props[1].name, "name".into());
    ///  assert_eq!(table.props[2].name, "age".into());
    ///  assert_eq!(table.annotations.len(), 1);
    ///  assert_eq!(table.annotations[0], TableAnnotation::Partition);
    /// ```
    pub fn add_annotation(mut self, annotation: TableAnnotation) -> Self {
        self.annotations.push(annotation);
        self
    }
}

impl From<Table> for Statement {
    fn from(table: Table) -> Self {
        Statement::Table(table)
    }
}

impl From<&Table> for Statement {
    fn from(table: &Table) -> Self {
        Statement::Table(table.clone())
    }
}

impl From<&str> for TableField {
    fn from(field: &str) -> Self {
        TableField::Text(Rc::from(field))
    }
}

impl TableProp {
    /// Create a new property
    /// # Example
    /// ```
    /// use cortex::objects::table::{TableProp, PropType};
    /// let prop = TableProp::new("id", PropType::Int32, None);
    /// assert_eq!(prop.name, "id".into());
    /// ```
    pub fn new(
        name: impl Into<TableField>,
        t_type: PropType,
        annotation: Option<PropAnnotation>,
    ) -> Self {
        TableProp {
            field: name.into(),
            field_type: t_type,
            annotation,
        }
    }
}

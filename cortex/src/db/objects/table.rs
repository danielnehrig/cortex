use std::rc::Rc;

use crate::objects::statement::Statement;

#[doc(alias = "Collection")]
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
/// TableProp struct for creating properties of a table
pub struct TableProp {
    pub field: TableField,
    pub field_type: PropType,
    pub annotation: Option<PropAnnotation>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableField {
    Text(Rc<str>),
    FieldAnnotation(FieldAnnotation),
}

impl TableField {
    pub fn get_as_text(&self) -> Option<String> {
        match self {
            TableField::Text(text) => Some(text.to_string()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum FieldAnnotation {
    PrimaryKey,
    ForeignKey(String, Table),
    Constraint(Box<FieldAnnotation>),
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
    ///    .add_prop_with_annotation(("id", PropType::Int32, None))
    ///    .add_prop_with_annotation(("name", PropType::Text, None))
    ///    .add_prop_with_annotation(("age", PropType::Int32, None));
    ///  assert_eq!(table.props.len(), 3);
    ///  assert_eq!(table.props[0].field, "id".into());
    ///  assert_eq!(table.props[1].field, "name".into());
    ///  assert_eq!(table.props[2].field, "age".into());
    /// ```
    pub fn add_prop_with_annotation(
        mut self,
        (name, prop_type, annotation): (&str, PropType, Option<PropAnnotation>),
    ) -> Self {
        self.props.push(TableProp::new(name, prop_type, annotation));
        self
    }

    /// Add a property to the table
    /// # Example
    /// ```
    /// use cortex::objects::table::{Table, TableProp, PropType};
    /// let table = Table::new("table")
    ///    .add_prop("id", PropType::Int32)
    ///    .add_prop("name", PropType::Text)
    ///    .add_prop("age", PropType::Int32);
    ///  assert_eq!(table.props.len(), 3);
    ///  assert_eq!(table.props[0].field, "id".into());
    ///  assert_eq!(table.props[1].field, "name".into());
    ///  assert_eq!(table.props[2].field, "age".into());
    /// ```
    pub fn add_prop(mut self, name: &str, ptype: PropType) -> Self {
        self.props.push(TableProp::new(name, ptype, None));
        self
    }

    /// Add a foreign key to the table
    /// # Disclaimer
    ///
    /// Make sure your DB in question does support relational tables
    ///
    /// - MongoDB  : does not support foreign keys no relation support
    pub fn add_foreign_key(mut self, name: &str, ptype: PropType, table: Table) -> Self {
        self.props
            .push(TableProp::new(TableField::Text(name.into()), ptype, None));
        self.props.push(TableProp::new(
            TableField::FieldAnnotation(FieldAnnotation::ForeignKey(name.into(), table)),
            PropType::Text,
            None,
        ));
        self
    }

    pub fn add_field_annotation(mut self, annotation: FieldAnnotation) -> Self {
        self.props.push(TableProp::new(
            TableField::FieldAnnotation(annotation),
            PropType::Text,
            None,
        ));
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
    ///  assert_eq!(table.props[0].field, "id".into());
    ///  assert_eq!(table.props[1].field, "name".into());
    ///  assert_eq!(table.props[2].field, "age".into());
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
    ///    .add_prop("id", PropType::Int32)
    ///    .add_prop("name", PropType::Text)
    ///    .add_prop("age", PropType::Int32)
    ///    .add_annotation(TableAnnotation::Partition);
    ///  assert_eq!(table.props.len(), 3);
    ///  assert_eq!(table.props[0].field, "id".into());
    ///  assert_eq!(table.props[1].field, "name".into());
    ///  assert_eq!(table.props[2].field, "age".into());
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
    /// assert_eq!(prop.field, "id".into());
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

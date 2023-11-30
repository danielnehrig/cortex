use std::rc::Rc;

#[doc(alias = "Collection")]
#[derive(Debug, Clone)]
/// Table struct for creating tables
pub struct Table {
    /// name of the table
    pub name: Rc<str>,
    /// properties of the table
    pub props: Vec<TableProp>,
    /// annotations of the table
    pub annotations: Vec<TableAnnotation>,
}

#[derive(Debug, Clone)]
/// TableProp struct for creating properties of a table
pub struct TableProp {
    pub name: Rc<str>,
    pub t_type: PropType,
    pub annotation: Option<PropAnnotation>,
}

#[derive(Debug, Clone)]
/// PropType enum for defining the type of a property
pub enum PropType {
    Int,
    Double,
    Text,
    Bool,
    Date,
    Timestamp,
    BigInt,
    SmallInt,
    // ...
}

#[derive(Debug, Clone, Default)]
/// PropAnnotation enum for defining the annotation of a property
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
    /// use cortex::producer::PostgresStatementProducer;
    /// use cortex::objects::table::{Table};
    /// let table: Table<PostgresStatementProducer> = Table::new("table");
    /// assert_eq!(table.name, "table".into());
    /// ```
    pub fn new(name: &str) -> Self {
        Table {
            name: Rc::from(name),
            props: Vec::new(),
            annotations: Vec::new(),
        }
    }

    /// Add a property to the table
    /// # Example
    /// ```
    /// use cortex::producer::PostgresStatementProducer;
    /// use cortex::objects::table::{Table, TableProp, PropType};
    /// let table: Table<PostgresStatementProducer> = Table::new("table")
    ///    .add_prop(TableProp::new("id", PropType::Int, None))
    ///    .add_prop(TableProp::new("name", PropType::Text, None))
    ///    .add_prop(TableProp::new("age", PropType::Int, None));
    ///  assert_eq!(table.props.len(), 3);
    ///  assert_eq!(table.props[0].name, "id".into());
    ///  assert_eq!(table.props[1].name, "name".into());
    ///  assert_eq!(table.props[2].name, "age".into());
    /// ```
    pub fn add_prop(mut self, prop: (&str, PropType, Option<PropAnnotation>)) -> Self {
        self.props.push(TableProp::new(prop.0, prop.1, prop.2));
        self
    }

    /// Add a properties to the table
    /// # Example
    /// ```
    /// use cortex::producer::PostgresStatementProducer;
    /// use cortex::objects::table::{Table, TableProp, PropType};
    /// let table: Table<PostgresStatementProducer> = Table::new("table")
    ///    .add_props_as_slice(&[
    ///    ("id", PropType::Int, None),
    ///    ("name", PropType::Text, None),
    ///    ("age", PropType::Int, None),
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
        for (name, t_type, annotation) in props {
            self.props
                .push(TableProp::new(name, t_type.clone(), annotation.clone()));
        }
        self
    }

    /// Add an annotation to the table
    /// # Example
    /// ```
    /// use cortex::producer::PostgresStatementProducer;
    /// use cortex::objects::table::{Table, TableProp, PropType, TableAnnotation};
    /// let table: Table<PostgresStatementProducer> = Table::new("table")
    ///    .add_prop(TableProp::new("id", PropType::Int, None))
    ///    .add_prop(TableProp::new("name", PropType::Text, None))
    ///    .add_prop(TableProp::new("age", PropType::Int, None))
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

impl TableProp {
    /// Create a new property
    /// # Example
    /// ```
    /// use cortex::producer::PostgresStatementProducer;
    /// use cortex::objects::table::{TableProp, PropType};
    /// let prop: TableProp<PostgresStatementProducer> = TableProp::new("id", PropType::Int, None);
    /// assert_eq!(prop.name, "id".into());
    /// ```
    pub fn new(name: &str, t_type: PropType, annotation: Option<PropAnnotation>) -> Self {
        TableProp {
            name: Rc::from(name),
            t_type,
            annotation,
        }
    }
}

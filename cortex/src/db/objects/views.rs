use crate::objects::table::{PropType, Table, TableProp};

#[derive(Clone, Debug)]
pub struct View {
    pub(crate) name: String,
    pub(crate) props: Vec<TableProp>,
    pub(crate) from: Vec<String>,
    pub(crate) where_clause: Vec<String>,
    #[allow(dead_code)]
    pub(crate) db: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ViewBuilder {
    name: String,
    props: Vec<TableProp>,
    from: Vec<String>,
    where_clause: Vec<String>,
    db: Option<String>,
}

impl ViewBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            props: Vec::new(),
            from: vec![],
            where_clause: vec![],
            db: None,
        }
    }

    pub fn from_table(&mut self, table: &Table) -> &mut Self {
        self.from.push(table.name.to_string());
        self.props.extend(table.props.clone());
        self
    }

    pub fn on_db(&mut self, db: &str) -> &mut Self {
        self.db = Some(db.into());
        self
    }

    pub fn add_prop(&mut self, name: &str, prop_type: PropType) -> &mut Self {
        self.props.push(TableProp {
            name: name.into(),
            t_type: prop_type,
            annotation: None,
        });
        self
    }

    pub fn add_from(&mut self, from: &str) -> &mut Self {
        self.from.push(from.into());
        self
    }

    pub fn add_where(&mut self, clause: &str) -> &mut Self {
        self.where_clause.push(clause.into());
        self
    }

    pub fn build(&self) -> View {
        if self.props.is_empty() {
            panic!(
                "View must have at least have one field/property to select specified by add_prop"
            );
        }

        if self.from.is_empty() {
            panic!("View must have at least one from clause to select data from table specified by add_from");
        }

        View {
            name: self.name.clone(),
            props: self.props.clone(),
            from: self.from.clone(),
            where_clause: self.where_clause.clone(),
            db: self.db.clone(),
        }
    }
}

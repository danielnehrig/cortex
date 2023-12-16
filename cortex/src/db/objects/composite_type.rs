use std::rc::Rc;

use crate::objects::table::PropType;

#[derive(Debug, Clone, PartialEq)]
/// TableProp struct for creating properties of a table
pub struct CompositeTypeProp {
    pub field: Rc<str>,
    pub field_type: PropType,
}

impl CompositeTypeProp {
    pub fn new(field: impl Into<Rc<str>>, field_type: PropType) -> Self {
        Self {
            field: field.into(),
            field_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeType {
    pub name: Rc<str>,
    pub props: Vec<CompositeTypeProp>,
}

impl CompositeType {
    pub fn new(name: Rc<str>) -> Self {
        Self {
            name,
            props: Vec::new(),
        }
    }

    pub fn add_prop(&mut self, field_name: impl Into<Rc<str>>, field_type: PropType) {
        self.props
            .push(CompositeTypeProp::new(field_name, field_type));
    }
}

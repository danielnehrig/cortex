use std::rc::Rc;

use crate::db::producer::DatabaseSpeicifics;

#[derive(Debug, Clone)]
pub struct Database<T: DatabaseSpeicifics> {
    pub name: Rc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DatabaseSpeicifics> Database<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: Rc::from(name),
            _marker: std::marker::PhantomData,
        }
    }
}

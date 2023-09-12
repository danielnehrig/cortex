use crate::db::producer::DatabaseSpeicifics;

#[derive(Debug, Clone)]
pub struct Database<'a, T: DatabaseSpeicifics> {
    pub name: &'a str,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: DatabaseSpeicifics> Database<'a, T> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            _marker: std::marker::PhantomData,
        }
    }
}

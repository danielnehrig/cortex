use crate::db::producer::DatabaseSpeicifics;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sequence<'a, T: DatabaseSpeicifics> {
    pub name: &'a str,
    pub start: i64,
    pub increment: i64,
    pub min_value: i64,
    pub max_value: i64,
    pub cycle: bool,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: DatabaseSpeicifics> Sequence<'a, T> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            start: 1,
            increment: 1,
            min_value: 1,
            max_value: 9223372036854775807,
            cycle: false,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn set_start(&mut self, start: i64) -> &mut Self {
        self.start = start;
        self
    }

    pub fn set_increment(&mut self, increment: i64) -> &mut Self {
        self.increment = increment;
        self
    }

    pub fn set_min_value(&mut self, min_value: i64) -> &mut Self {
        self.min_value = min_value;
        self
    }

    pub fn set_max_value(&mut self, max_value: i64) -> &mut Self {
        self.max_value = max_value;
        self
    }

    pub fn set_cycle(&mut self, cycle: bool) -> &mut Self {
        self.cycle = cycle;
        self
    }
}

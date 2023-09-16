use std::fmt::Display;

use crate::db::producer::DatabaseSpeicifics;

#[derive(Clone)]
pub enum Statement<'a, T: DatabaseSpeicifics> {
    Create(&'a dyn CreateableObject),
    Drop(&'a dyn DropableObject),
    Alter(&'a dyn AlterableOject),
    Insert(&'a dyn Display),
    _Phantom(std::marker::PhantomData<T>),
}

pub trait CreateableObject: Display {
    fn create(&self) -> String;
}

pub trait InsertableObject: Display {
    fn insert(&self) -> String;
}

pub trait DropableObject: Display {
    fn drop(&self) -> String;
}

pub trait AlterableOject: Display {
    fn alter(&self) -> String;
}

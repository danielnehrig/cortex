use std::rc::Rc;

use crate::objects::{procedure::StoredProcedure, table::Table, views::View};

#[derive(Clone, Debug, PartialEq)]
pub enum TriggerForEach {
    Row,
    Statement,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TriggerWhen;

#[derive(Clone, Debug, PartialEq)]
pub enum TriggerAction {
    Insert,
    Update,
    Delete,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TriggerTime {
    Before,
    After,
    InsteadOf,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TriggerEvent {
    action: TriggerAction,
    time: TriggerTime,
    on: TriggerEventOn,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TriggerEventOn {
    Table(Table),
    View(View),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Trigger {
    name: Rc<str>,
    event: TriggerEvent,
    for_each: TriggerForEach,
    when: Option<TriggerWhen>,
    execute: Rc<str>,
}

impl Trigger {
    pub fn new(
        name: impl Into<Rc<str>>,
        event: TriggerEvent,
        for_each: TriggerForEach,
        function: StoredProcedure,
    ) -> Self {
        Self {
            name: name.into(),
            event,
            for_each,
            when: None,
            execute: Rc::from(function.name.as_str()),
        }
    }
}

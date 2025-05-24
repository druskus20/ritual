use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Day {
    pub id: Uuid,
    pub date: DateTime,
    pub habits: IndexMap<Uuid, HabitRef>,
}

impl Day {
    pub fn new(date: DateTime) -> Self {
        Self {
            id: Uuid::new_v4(),
            date,
            habits: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Habit {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct HabitRef {
    pub id: Uuid,
    pub name: String,
    pub done: bool,
}

impl PartialEq for HabitRef {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for HabitRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl std::hash::Hash for Habit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Habit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

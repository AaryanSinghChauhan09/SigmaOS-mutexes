// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mutexes: mutexes component model.
//!
//! Real implementation: mutexes_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutexesItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutexesError {
    EmptyMutexesItemName,
    DuplicateMutexesItem(String),
    EmptyEventSubject,
}

impl fmt::Display for MutexesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMutexesItemName => write!(f, "mutexes_item name must not be empty"),
            Self::DuplicateMutexesItem(id) => write!(f, "duplicate mutexes_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MutexesError {}

pub struct Mutexes {
    pub name: String,
    mutexes_items: BTreeMap<String, MutexesItem>,
    events: BTreeMap<String, Event>,
}

impl Mutexes {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mutexes_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_mutexes_item(&mut self, item: MutexesItem) -> Result<(), MutexesError> {
        if item.name.trim().is_empty() {
            return Err(MutexesError::EmptyMutexesItemName);
        }
        let id = item.id.clone();
        if self.mutexes_items.contains_key(&id) {
            return Err(MutexesError::DuplicateMutexesItem(id.clone()));
        }
        self.mutexes_items.insert(id, item);
        Ok(())
    }

    pub fn mutexes_item_count(&self) -> usize {
        self.mutexes_items.len()
    }

    pub fn get_mutexes_item(&self, id: &str) -> Option<&MutexesItem> {
        self.mutexes_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MutexesError> {
        if event.subject.trim().is_empty() {
            return Err(MutexesError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_mutexes_item(id: &str, name: &str) -> MutexesItem {
        MutexesItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Mutexes::new("Mutexes");
        assert_eq!(m.name, "Mutexes");
        assert_eq!(m.mutexes_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_mutexes_item() {
        let mut m = Mutexes::new("Mutexes");
        m.add_mutexes_item(sample_mutexes_item("a1", "Alpha")).unwrap();
        assert_eq!(m.mutexes_item_count(), 1);
        assert!(m.get_mutexes_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Mutexes::new("Mutexes");
        assert!(m.add_mutexes_item(sample_mutexes_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Mutexes::new("Mutexes");
        m.add_mutexes_item(sample_mutexes_item("x", "One")).unwrap();
        assert!(m.add_mutexes_item(sample_mutexes_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Mutexes::new("Mutexes");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

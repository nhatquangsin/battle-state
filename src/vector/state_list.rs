use crate::vector::{State, Path};
use crate::gen::Reader;
use crate::CardState;
use std::sync::Arc;

pub struct StateList {
    pub path: Path,
    pub items: Vec<Option<Arc<CardState>>>,
}

impl StateList
{
    pub fn add(&mut self, reader: &mut Reader) {
        let index = self.items.len() as u16;

        let path = self.path.derive(index);
        self.items.push(Some(Arc::new(CardState::new(None).deserialize(reader, Some(path)))));
    }

    pub fn remove(&mut self, reader: &mut Reader) {
        let index = reader.next_u16() as usize;

        if (index) < self.items.len() {
            self.items[index] = None;
        }
    }

    pub fn new(path: Option<Path>) -> Self {
        let mut new_path: Path = Path::new();
        if let Some(check_path) = path {
            new_path = check_path;
        }

        Self {
            path: new_path,
            items: vec![],
        }
    }

    fn deserialize(&mut self, reader: &mut Reader, path: Option<Path>) -> Self {
        let mut list = StateList::new(path);
        let length = reader.next_u16();
        let size = reader.next_u16();

        for i in 0..length {
            list.items.push(None);
        }

        for i in 0..size {
            list.replace_at(reader);
        }

        list
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        let index = reader.next_u16() as usize;

        if (index) >= self.items.len() {
            for i in self.items.len()..(index + 1) {
                self.items.push(None);
            }
        }

        let path = self.path.derive(index as u16);
        self.items[index] = Some(Arc::new(CardState::new(None).deserialize(reader, Some(path))));
    }

    fn nested(&self, index: u16) -> Option<Arc<CardState>> {
        if (index as usize) >= self.items.len() {
            return None;
        }
        if let Some(item_ref) = self.items[index as usize].as_ref() {
            return Some(Arc::clone(item_ref));
        }
        None
    }
}

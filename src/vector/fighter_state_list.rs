use crate::vector::{State, Path};
use crate::gen::Reader;
use crate::states::{FighterState, StateTypes};

pub struct FighterStateList {
    pub path: Path,
    pub items: Vec<Option<FighterState>>,
}

impl FighterStateList
{
    pub fn add(&mut self, reader: &mut Reader) {
        let index = self.items.len() as u16;

        let path = self.path.derive(index);
        self.items.push(Some(FighterState::deserialize(reader, Some(path))));
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

    pub fn deserialize(reader: &mut Reader, path: Option<Path>) -> Self {
        let mut list = FighterStateList::new(path);
        let length = reader.next_u16();
        let size = reader.next_u16();
        println!("fighter state list deserialize");

        for i in 0..length {
            list.items.push(None);
        }

        for i in 0..size {
            list.replace_at(reader);
        }

        list
    }

    pub fn replace_at(&mut self, reader: &mut Reader) {
        let index = reader.next_u16() as usize;
        println!("fighter state list replace at {}", index);

        if (index) >= self.items.len() {
            for i in self.items.len()..(index + 1) {
                self.items.push(None);
            }
        }

        let path = self.path.derive(index as u16);
        self.items[index] = Some(FighterState::deserialize(reader, Some(path)));
    }

    pub fn nested(&mut self, index: u16) -> Option<StateTypes> {
        if (index as usize) < self.items.len() {
            if let Some(item) = &mut self.items[index as usize] {
                return Some(StateTypes::FighterState(item));
            }
        }
        None
    }
}

use crate::vector::{State, Path};
use crate::gen::Reader;
use crate::states::StateTypes;

pub struct I32List {
    pub path: Path,
    pub items: Vec<Option<i32>>,
}

impl I32List
{
    pub fn add(&mut self, reader: &mut Reader) {
        let index = self.items.len() as u16;

        let path = self.path.derive(index);
        self.items.push(Some(reader.next_i32()));
    }

    pub fn remove(&mut self, reader: &mut Reader) {
        let index = reader.next_u16() as usize;

        if index < self.items.len() {
            let value = self.items[index];

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
        let mut list = I32List::new(path);
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

    pub fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16() as usize;

            if index >= self.items.len() {
                for i in self.items.len()..(index + 1) {
                    self.items.push(None);
                }
            }

            let path = self.path.derive(index as u16);
            self.items[index] = Some(reader.next_i32());
        }
    }

    pub fn nested(&mut self, reader: &mut Reader, path_length: u16) -> StateTypes {
        if path_length == 0 {
            return StateTypes::I32List(self);
        }
        StateTypes::None
    }
}

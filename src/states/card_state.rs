use crate::gen::Reader;
use crate::vector::{State, Path, TypeInfo};

pub struct CardState {
    pub path: Path,

    pub card_id: i32,
    pub card_type: i32,
    pub energy: i32,
    pub disabled: bool,
    pub attack: i32,
    pub defense: i32,
    pub unique_id: i32,
}

impl CardState {
    pub fn new(path: Option<Path>) -> Self {
        let mut new_path: Path = Path::new();
        if let Some(checked_path) = path {
            new_path = checked_path;
        }

        Self {
            path: new_path,
            card_id: 0,
            card_type: 0,
            energy: 0,
            disabled: false,
            attack: 0,
            defense: 0,
            unique_id: 0,
        }
    }
}

impl State for CardState {
    fn deserialize(&mut self, reader: &mut Reader, path: Option<Path>) -> Self {
        let mut card_state = Self::new(path);
        let length = reader.next_u16();

        for _i in 0..length {
            card_state.replace_at(reader);
        }

        card_state
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16();

            match index {
                0 => self.card_id = reader.next_i32(),
                1 => self.card_type = reader.next_i32(),
                9 => self.energy = reader.next_i32(),
                10 => self.disabled = reader.next_bool(),
                2 => self.attack = reader.next_i32(),
                3 => self.defense = reader.next_i32(),
                4 => self.unique_id = reader.next_i32(),
                _ => {},
            }
        }
    }

    fn nested(&self, index: u16) -> Option<Self> {
        match index {
            _ => None,
        }
    }
}


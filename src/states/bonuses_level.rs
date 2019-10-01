use crate::gen::Reader;
use crate::vector::{State, Path};
use crate::states::state_types::StateTypes;

pub struct BonusesLevel {
    pub path: Path,

    pub attack: i32,
    pub speed: i32,
    pub morale: i32,
}

impl BonusesLevel {
    pub fn new(path: Option<Path>) -> Self {
        let mut new_path: Path = Path::new();
        if let Some(checked_path) = path {
            new_path = checked_path;
        }

        Self {
            path: new_path,
            attack: 0,
            speed: 0,
            morale: 0,
        }
    }
}

impl State for BonusesLevel {
    fn deserialize(reader: &mut Reader, path: Option<Path>) -> Self {
        let mut bonuses_level = Self::new(path);
        let length = reader.next_u16();

        for _i in 0..length {
            bonuses_level.replace_at(reader);
        }

        bonuses_level
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16();

            match index {
                0 => self.attack = reader.next_i32(),
                1 => self.speed = reader.next_i32(),
                2 => self.morale = reader.next_i32(),
                _ => {},
            }
        }
    }

    fn nested(&mut self, reader: &mut Reader, path_length: u16) -> StateTypes {
        if path_length == 0 {
            return StateTypes::BonusesLevel(self);
        }
        StateTypes::None
    }
}


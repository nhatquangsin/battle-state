use crate::gen::Reader;
use crate::vector::{State, Path, TypeInfo};

pub struct BuffState {
    pub path: Path,

    pub id: i32,
    pub debuff: bool,
    pub rounds_left: i32,
    pub turns_left: i32,
    pub bonuses: BonusesLevel,
    pub fighters: StateList<i32>,
}

impl BuffState {
    pub fn new(path: Option<Path>) -> Self {
        let mut new_path: Path = Path::new();
        if let Some(checked_path) = path {
            new_path = checked_path;
        }

        Self {
            path: new_path,
            id: 0,
            debuff: false,
            rounds_left: 0,
            turns_left: 0,
            bonuses: None,
            fighters: vec![],
        }
    }
}

impl State for BuffState {
    fn deserialize(&mut self, reader: &mut Reader, path: Option<Path>) -> Self {
        let mut buff_state = Self::new(path);
        let length = reader.next_u16();

        for _i in 0..length {
            card_state.replace_at(reader);
        }

        buff_state
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16();

            match index {
                0 => self.id = reader.next_i32(),
                1 => self.debuff = reader.next_bool(),
                2 => self.rounds_left = reader.next_i32(),
                3 => self.turns_left = reader.next_i32(),
                4 => self.bonuses = BonusesLevel.deserialize(reader, self.path.derive(4))
                5 => self.fighters = StateList<i32>.deserialize(reader, self.path.derive(5))
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


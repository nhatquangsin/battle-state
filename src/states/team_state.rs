use crate::gen::Reader;
use crate::vector::{State, Path, TypeInfo};

pub struct TeamState {
    pub path: Path,

    pub fighters: StateList<FighterState>,
    pub secret_card_deck: StateList<i32>,
    pub active_card_deck: StateList<CardState>,
    pub discard_card_deck: StateList<i32>,
    pub energy: i32,
    pub surrendered: bool,
    pub index: i32,
}

impl TeamState {
    pub fn new(path: Option<Path>) -> Self {
        let mut new_path: Path = Path::new();
        if let Some(checked_path) = path {
            new_path = checked_path;
        }

        Self {
            path: new_path,
            fighters: vec![],
            secret_card_deck: vec![],
            active_card_deck: vec![],
            discard_card_deck: vec![],
            energy: 0,
            surrendered: false,
            index: 0,
        }
    }
}

impl State for TeamState {
    fn deserialize(&mut self, reader: &mut Reader, path: Option<Path>) -> Self {
        let mut team_state = Self::new(path);
        let length = reader.next_u16();

        for _i in 0..length {
            card_state.replace_at(reader);
        }

        team_state
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16();

            match index {
                0 => self.fighters = StateList<FighterState>.deserialize(reader, self.path.derive(0))
                1 => self.secret_card_deck = StateList<i32>.deserialize(reader, self.path.derive(1))
                2 => self.active_card_deck = StateList<CardState>.deserialize(reader, self.path.derive(2))
                3 => self.discard_card_deck = StateList<i32>.deserialize(reader, self.path.derive(3))
                4 => self.energy = reader.next_i32(),
                5 => self.surrendered = reader.next_bool(),
                6 => self.index = reader.next_i32(),
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


use crate::gen::Reader;
use crate::vector::{State, Path, TypeInfo};

pub struct RoundState {
    pub path: Path,

    pub phase: i32,
    pub first_player_cards: StateList<CardState>,
    pub first_player_ready: bool,
    pub second_player_cards: StateList<CardState>,
    pub second_player_ready: bool,
    pub speed_order: StateList<i32>,
    pub card_order: StateList<i32>,
    pub triggered_effects: StateList<i32>,
}

impl RoundState {
    pub fn new(path: Option<Path>) -> Self {
        let mut new_path: Path = Path::new();
        if let Some(checked_path) = path {
            new_path = checked_path;
        }

        Self {
            path: new_path,
            phase: 0,
            first_player_cards: vec![],
            first_player_ready: false,
            second_player_cards: vec![],
            second_player_ready: false,
            speed_order: vec![],
            card_order: vec![],
            triggered_effects: vec![],
        }
    }
}

impl State for RoundState {
    fn deserialize(&mut self, reader: &mut Reader, path: Option<Path>) -> Self {
        let mut round_state = Self::new(path);
        let length = reader.next_u16();

        for _i in 0..length {
            card_state.replace_at(reader);
        }

        round_state
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16();

            match index {
                0 => self.phase = reader.next_i32(),
                1 => self.first_player_cards = StateList<CardState>.deserialize(reader, self.path.derive(1))
                2 => self.first_player_ready = reader.next_bool(),
                3 => self.second_player_cards = StateList<CardState>.deserialize(reader, self.path.derive(3))
                4 => self.second_player_ready = reader.next_bool(),
                5 => self.speed_order = StateList<i32>.deserialize(reader, self.path.derive(5))
                6 => self.card_order = StateList<i32>.deserialize(reader, self.path.derive(6))
                7 => self.triggered_effects = StateList<i32>.deserialize(reader, self.path.derive(7))
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


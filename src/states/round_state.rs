use crate::gen::Reader;
use crate::vector::{State, Path};
use crate::state_list::{CardStateList, I32List};
use crate::states::state_types::StateTypes;

pub struct RoundState {
    pub path: Path,

    pub phase: i32,
    pub first_player_cards: CardStateList,
    pub first_player_ready: bool,
    pub second_player_cards: CardStateList,
    pub second_player_ready: bool,
    pub speed_order: I32List,
    pub card_order: I32List,
    pub triggered_effects: I32List,
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
            first_player_cards: CardStateList::new(None),
            first_player_ready: false,
            second_player_cards: CardStateList::new(None),
            second_player_ready: false,
            speed_order: I32List::new(None),
            card_order: I32List::new(None),
            triggered_effects: I32List::new(None),
        }
    }
}

impl State for RoundState {
    fn deserialize(reader: &mut Reader, path: Option<Path>) -> Self {
        let mut round_state = Self::new(path);
        let length = reader.next_u16();

        for _i in 0..length {
            round_state.replace_at(reader);
        }

        round_state
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16();

            match index {
                0 => self.phase = reader.next_i32(),
                1 => self.first_player_cards = CardStateList::deserialize(reader, Some(self.path.derive(1))),
                2 => self.first_player_ready = reader.next_bool(),
                3 => self.second_player_cards = CardStateList::deserialize(reader, Some(self.path.derive(3))),
                4 => self.second_player_ready = reader.next_bool(),
                5 => self.speed_order = I32List::deserialize(reader, Some(self.path.derive(5))),
                6 => self.card_order = I32List::deserialize(reader, Some(self.path.derive(6))),
                7 => self.triggered_effects = I32List::deserialize(reader, Some(self.path.derive(7))),
                _ => {},
            }
        }
    }

    fn nested(&mut self, reader: &mut Reader, path_length: u16) -> StateTypes {
        if path_length == 0 {
            return StateTypes::RoundState(self);
        }
        let index = reader.next_u16();
        match index {
            1 => self.first_player_cards.nested(reader, path_length - 1),
            3 => self.second_player_cards.nested( reader, path_length - 1),
            5 => self.speed_order.nested( reader, path_length - 1),
            6 => self.card_order.nested(reader, path_length - 1),
            7 => self.triggered_effects.nested( reader, path_length - 1),
            _ => StateTypes::None,
        }
    }
}


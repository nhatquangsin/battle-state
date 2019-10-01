use crate::gen::Reader;
use crate::vector::{State, Path, I32List, CardStateList, FighterStateList};
use crate::states::state_types::StateTypes;

pub struct TeamState {
    pub path: Path,

    pub fighters: FighterStateList,
    pub secret_card_deck: I32List,
    pub active_card_deck: CardStateList,
    pub discard_card_deck: I32List,
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
            fighters: FighterStateList::new(None),
            secret_card_deck: I32List::new(None),
            active_card_deck: CardStateList::new(None),
            discard_card_deck: I32List::new(None),
            energy: 0,
            surrendered: false,
            index: 0,
        }
    }
}

impl State for TeamState {
    fn deserialize(reader: &mut Reader, path: Option<Path>) -> Self {
        let mut team_state = Self::new(path);
        let length = reader.next_u16();
        println!("      team length {}", length);
        if length != 1 && length != 7 {
            panic!("team hello {}", length);
        }

        for _i in 0..length {
            team_state.replace_at(reader);
        }

        team_state
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16();
            println!("      team index {}", index);

            match index {
                0 => self.fighters = FighterStateList::deserialize(reader, Some(self.path.derive(0))),
                1 => self.secret_card_deck = I32List::deserialize(reader, Some(self.path.derive(1))),
                2 => self.active_card_deck = CardStateList::deserialize(reader, Some(self.path.derive(2))),
                3 => self.discard_card_deck = I32List::deserialize(reader, Some(self.path.derive(3))),
                4 => self.energy = reader.next_i32(),
                5 => self.surrendered = reader.next_bool(),
                6 => self.index = reader.next_i32(),
                _ => {},
            }
        }
    }

    fn nested(&mut self, reader: &mut Reader, path_length: u16) -> StateTypes {
        if path_length == 0 {
            return StateTypes::TeamState(self);
        }
        let index = reader.next_u16();
        match index {
            0 => self.fighters.nested(reader, path_length - 1),
            1 => self.secret_card_deck.nested( reader, path_length - 1),
            2 => self.active_card_deck.nested(reader, path_length - 1),
            3 => self.discard_card_deck.nested( reader, path_length - 1),
            _ => StateTypes::None,
        }
    }
}


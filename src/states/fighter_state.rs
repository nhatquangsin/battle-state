use crate::gen::Reader;
use crate::vector::{State, Path, TypeInfo};
use crate::states::state_types::StateTypes;

#[derive(Debug)]
pub struct FighterState {
    pub path: Path,

    pub fighter_id: i32,
    pub team_id: i32,
    pub position: i32,
    pub hp: i32,
    pub shield: i32,
    pub last_stand_turns: i32,
}

impl FighterState {
    pub fn new(path: Option<Path>) -> Self {
        let mut new_path: Path = Path::new();
        if let Some(checked_path) = path {
            new_path = checked_path;
        }

        Self {
            path: new_path,
            fighter_id: 0,
            team_id: 0,
            position: 0,
            hp: 0,
            shield: 0,
            last_stand_turns: 0,
        }
    }
}

impl State for FighterState {
    fn deserialize(reader: &mut Reader, path: Option<Path>) -> Self {
        let mut fighter_state = Self::new(path);
        let length = reader.next_u16();
        println!("          fighter state deserialize");
        if length != 1 && length != 6 {
            panic!("hello {}", length);
        }

        for _i in 0..length {
            fighter_state.replace_at(reader);
        }

        fighter_state
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16();
            println!("          fighter state replace at {}", index);

            match index {
                0 => self.fighter_id = reader.next_i32(),
                1 => self.team_id = reader.next_i32(),
                2 => self.position = reader.next_i32(),
                3 => self.hp = reader.next_i32(),
                4 => self.shield = reader.next_i32(),
                5 => self.last_stand_turns = reader.next_i32(),
                _ => {},
            }
        }
    }

    fn nested(&mut self, reader: &mut Reader, path_length: u16) -> StateTypes {
        if path_length == 0 {
            return StateTypes::FighterState(self);
        }
        StateTypes::None
    }
}


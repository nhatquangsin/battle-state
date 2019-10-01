use crate::gen::Reader;
use crate::vector::{State, Path, TypeInfo};
use crate::states::state_types::StateTypes;

pub struct TurnState {
    pub path: Path,

    pub unique_id: i32,
    pub turn_index: i32,
    pub phase: i32,
    pub attacker: i32,
    pub defender: i32,
    pub should_hit: bool,
    pub should_crit: bool,
    pub crit_multiplier: i32,
    pub hit_damage: i32,
    pub hit_times: i32,
    pub shield_broke: bool,
    pub should_counter: bool,
    pub counter_damage: i32,
}

impl TurnState {
    pub fn new(path: Option<Path>) -> Self {
        let mut new_path: Path = Path::new();
        if let Some(checked_path) = path {
            new_path = checked_path;
        }

        Self {
            path: new_path,
            unique_id: 0,
            turn_index: 0,
            phase: 0,
            attacker: 0,
            defender: 0,
            should_hit: false,
            should_crit: false,
            crit_multiplier: 0,
            hit_damage: 0,
            hit_times: 0,
            shield_broke: false,
            should_counter: false,
            counter_damage: 0,
        }
    }
}

impl State for TurnState {
    fn deserialize(reader: &mut Reader, path: Option<Path>) -> Self {
        let mut turn_state = Self::new(path);
        let length = reader.next_u16();

        for _i in 0..length {
            turn_state.replace_at(reader);
        }

        turn_state
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16();

            match index {
                0 => self.unique_id = reader.next_i32(),
                1 => self.turn_index = reader.next_i32(),
                2 => self.phase = reader.next_i32(),
                3 => self.attacker = reader.next_i32(),
                4 => self.defender = reader.next_i32(),
                5 => self.should_hit = reader.next_bool(),
                6 => self.should_crit = reader.next_bool(),
                7 => self.crit_multiplier = reader.next_i32(),
                8 => self.hit_damage = reader.next_i32(),
                9 => self.hit_times = reader.next_i32(),
                10 => self.shield_broke = reader.next_bool(),
                11 => self.should_counter = reader.next_bool(),
                12 => self.counter_damage = reader.next_i32(),
                _ => {},
            }
        }
    }

    fn nested(&mut self, reader: &mut Reader, path_length: u16) -> StateTypes {
        if path_length == 0 {
            return StateTypes::TurnState(self);
        }
        StateTypes::None
    }
}


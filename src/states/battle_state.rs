use crate::gen::Reader;
use crate::vector::{State, Path, TypeInfo};

pub struct BattleState {
    pub path: Path,

    pub first_team: TeamState,
    pub second_team: TeamState,
    pub buffs: StateList<BuffState>,
    pub round: RoundState,
    pub turn: TurnState,
    pub winner: i32,
}

impl BattleState {
    pub fn new(path: Option<Path>) -> Self {
        let mut new_path: Path = Path::new();
        if let Some(checked_path) = path {
            new_path = checked_path;
        }

        Self {
            path: new_path,
            first_team: None,
            second_team: None,
            buffs: vec![],
            round: None,
            turn: None,
            winner: 0,
        }
    }
}

impl State for BattleState {
    fn deserialize(&mut self, reader: &mut Reader, path: Option<Path>) -> Self {
        let mut battle_state = Self::new(path);
        let length = reader.next_u16();

        for _i in 0..length {
            card_state.replace_at(reader);
        }

        battle_state
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        if !reader.eof() {
            let index = reader.next_u16();

            match index {
                0 => self.first_team = TeamState.deserialize(reader, self.path.derive(0))
                1 => self.second_team = TeamState.deserialize(reader, self.path.derive(1))
                2 => self.buffs = StateList<BuffState>.deserialize(reader, self.path.derive(2))
                3 => self.round = RoundState.deserialize(reader, self.path.derive(3))
                4 => self.turn = TurnState.deserialize(reader, self.path.derive(4))
                5 => self.winner = reader.next_i32(),
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


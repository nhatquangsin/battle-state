use crate::gen::Reader;
use crate::vector::{State, Path, BuffStateList};
use crate::states::{TeamState, RoundState, TurnState};
use crate::states::state_types::StateTypes;

pub struct BattleState {
    pub path: Path,

    pub first_team: TeamState,
    pub second_team: TeamState,
    pub buffs: BuffStateList,
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
            first_team: TeamState::new(None),
            second_team: TeamState::new(None),
            buffs: BuffStateList::new(None),
            round: RoundState::new(None),
            turn: TurnState::new(None),
            winner: 0,
        }
    }
}

impl State for BattleState {
    fn deserialize(reader: &mut Reader, path: Option<Path>) -> Self {
        let mut battle_state = Self::new(path);
        let length = reader.next_u16();
        if length != 1 && length != 6 {
            panic!("battle hello {}", length);
        }

        for _i in 0..length {
            battle_state.replace_at(reader);
        }

        battle_state
    }

    fn replace_at(&mut self, reader: &mut Reader) {
        let index = reader.next_u16();
        println!("battle index {}", index);

        match index {
            0 => self.first_team = TeamState::deserialize(reader, Some(self.path.derive(0))),
            1 => self.second_team = TeamState::deserialize(reader, Some(self.path.derive(1))),
            2 => self.buffs = BuffStateList::deserialize(reader, Some(self.path.derive(2))),
            3 => self.round = RoundState::deserialize(reader, Some(self.path.derive(3))),
            4 => self.turn = TurnState::deserialize(reader, Some(self.path.derive(4))),
            5 => self.winner = reader.next_i32(),
            _ => {},
        }
    }

    fn nested(&mut self, reader: &mut Reader, path_length: u16) -> StateTypes {
        if path_length == 0 {
            return StateTypes::BattleState(self);
        }
        let index = reader.next_u16();
        match index {
            0 => self.first_team.nested( reader, path_length - 1),
            1 => self.second_team.nested( reader, path_length - 1),
            2 => self.buffs.nested( reader, path_length - 1),
            3 => self.round.nested( reader, path_length - 1),
            4 => self.turn.nested( reader, path_length - 1),
            _ => StateTypes::None,
        }
    }
}


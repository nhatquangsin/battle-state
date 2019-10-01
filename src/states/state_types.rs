use crate::states::{BattleState, BonusesLevel, BuffState, CardState, FighterState, RoundState, TeamState, TurnState};
use crate::vector::{BuffStateList, FighterStateList, I32List, CardStateList, BoolStateList};

pub enum StateTypes<'a> {
    BattleState(&'a mut BattleState),
    BonusesLevel(&'a mut BonusesLevel),
    BuffState(&'a mut BuffState),
    CardState(&'a mut CardState),
    FighterState(&'a mut FighterState),
    RoundState(&'a mut RoundState),
    TeamState(&'a mut TeamState),
    TurnState(&'a mut TurnState),
    BuffStateList(&'a mut BuffStateList),
    FighterStateList(&'a mut FighterStateList),
    I32List(&'a mut I32List),
    I32(&'a mut i32),
    BoolList(&'a mut BoolStateList),
    CardStateList(&'a mut CardStateList),
    None
}
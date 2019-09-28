pub mod gen;
pub mod states;
pub mod vector;

use gen::Reader;
use vector::Path;
use vector::StateList;
use states::CardState;
use crate::vector::{State, TypeInfo};

fn main() {
    let data: Vec<u8> = vec![0,0,0,0,1,0,3,0,0,0,0,0,1,0,4,0,0,0,0,1,0,3,0,0,0,0,0,2,0,4,0,0,0,0,1,0,3,0,1,0,0,0,0,0,4,0,0,0,0,1,0,3,0,1,0,0,0,1,0,4,0,0,0,0];
//    let mut reader = Reader::new(data);
//    println!("{}", reader.next_byte());
//    println!("{}", reader.next_byte());
//    println!("{}", Path::ROOT.index);
    let mut reader = Reader::new(data);
    let mut card_state = CardState::new(None);

    let mut c = 0;
    let mut state: CardState = CardState::new(None);
    while !reader.eof() {
        state = card_state.deserialize(&mut reader, None);
        c = c + 1;
    }
    println!("card state - attack: {}", state.attack);
    println!("card state - unique id: {}", state.unique_id);
    println!("card state - card id: {}", state.card_id);
    println!("card state - energy: {}", state.energy);
    println!("card state - defense: {}", state.defense);
    println!("card state - card type: {}", state.card_type);
}

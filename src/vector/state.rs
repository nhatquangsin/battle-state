use crate::Reader;
use crate::Path;
use crate::states::StateTypes;

pub trait State {
    fn deserialize(reader: &mut Reader, path: Option<Path>) -> Self;
    fn replace_at(&mut self, reader: &mut Reader);
    fn nested(&mut self, index: u16) -> Option<StateTypes> where Self: std::marker::Sized;
}

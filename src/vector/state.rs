use crate::Reader;
use crate::Path;

pub trait State {
    fn deserialize(&mut self, reader: &mut Reader, path: Option<Path>) -> Self;
    fn replace_at(&mut self, reader: &mut Reader);
    fn nested(&self, index: u16) -> Option<Self> where Self: std::marker::Sized;
}

use std::ops::{Index, IndexMut};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(
    Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromRepr, EnumIter, EnumCount,
)]
pub enum Item {
    Trap,
    Pickaxe,
    Carrot,
    Shovel,
}

impl<T> Index<Item> for [T; Item::COUNT] {
    type Output = T;

    fn index(&self, index: Item) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Item> for [T; Item::COUNT] {
    fn index_mut(&mut self, index: Item) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

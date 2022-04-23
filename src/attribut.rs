elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum Background {
    None,
    White,
    Black,
    Yellow,
}

#[derive(TypeAbi, NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum Skin {
    None,
    White,
    Black,
    Yellow,
}

#[derive(TypeAbi, NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum Hat {
    None,
    White,
    Black,
    Yellow,
}

#[derive(TypeAbi, NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum Accessory {
    Gun,
    Flower,
    Axe,
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct NftAttributes {
    pub background: Background,
    pub skin: Skin,
    pub hat: Hat,
    pub accessories: Option<Accessory>,
}

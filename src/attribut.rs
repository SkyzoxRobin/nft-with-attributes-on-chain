elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct NftAttributes<M: ManagedTypeApi> {
    pub background: ManagedBuffer<M>,
    pub skin: ManagedBuffer<M>,
    pub hat: ManagedBuffer<M>,
    pub accessories : Option<ManagedBuffer<M>>
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct NftAttributesEnum {
    pub background: Background,
    pub skin: Skin,
    pub hat: Hat,
    pub accessories: Option<Accessory>,
}

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

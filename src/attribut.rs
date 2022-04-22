elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct NftAttributes<M: ManagedTypeApi> {
    pub background: ManagedBuffer<M>,
    pub skin: ManagedBuffer<M>,
    pub hat: ManagedBuffer<M>,
    pub accessories : Option<ManagedBuffer<M>>
}
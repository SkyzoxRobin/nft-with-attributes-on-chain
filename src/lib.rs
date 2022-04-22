#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod attribut;
use attribut::NftAttributes;

#[elrond_wasm::contract]
pub trait NftOnChain {
    #[init]
    fn init(
        &self,
        name: ManagedBuffer,
        image_cid: ManagedBuffer,
        metadata_cid: ManagedBuffer
    ) {
        self.name().set(&name);
        self.image_cid().set(&image_cid);
        self.metadata_cid().set(&metadata_cid);
    }

    #[only_owner]
    #[endpoint(fillAttributes)]
    fn fill_attributes_endpoint(
        &self,
        number: u64,
        background: ManagedBuffer,
        skin: ManagedBuffer,
        hat: ManagedBuffer,
        accessories: Option<ManagedBuffer>
    ) {
        let attributes = NftAttributes {
            background: background,
            skin: skin,
            hat: hat,
            accessories: accessories
        };
        self.attributes(number).set(&attributes);
    }

    #[storage_mapper("attributes")]
    fn attributes(&self, number: u64) -> SingleValueMapper<NftAttributes<Self::Api>>;

    #[storage_mapper("name")]
    fn name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("imageCid")]
    fn image_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("metadataCid")]
    fn metadata_cid(&self) -> SingleValueMapper<ManagedBuffer>;
}

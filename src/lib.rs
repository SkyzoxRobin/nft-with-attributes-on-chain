#![no_std]
extern crate alloc;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const IPFS_GATEWAY: &[u8] = "https://ipfs.io/ipfs/".as_bytes();
const ROYALTIES: u64 = 500;

mod attribut;
mod token;
use alloc::string::ToString;
use attribut::*;

pub type AttributesAsMultiValue<M> = 
    MultiValue5<u64, ManagedBuffer<M>, ManagedBuffer<M>, ManagedBuffer<M>, Option<ManagedBuffer<M>>>;

pub type AttributesAsMultiValueEnum = 
    MultiValue5<u64, Background, Skin, Hat, Option<Accessory>>;

#[elrond_wasm::contract]
pub trait NftWithOnChainAttributs: token::TokenModule {
    #[init]
    fn init(&self, name: ManagedBuffer, image_cid: ManagedBuffer, metadata_cid: ManagedBuffer) {
        self.name().set(&name);
        self.image_cid().set(&image_cid);
        self.metadata_cid().set(&metadata_cid);
    }

    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(&self, number_to_mint: u64) {
        let token = self.nft_token_id().get();
        let royalties = BigUint::from(ROYALTIES);
        let name = self.name().get();

        require!(
            !self.attributes(number_to_mint).is_empty(),
            "On-chain attributes for this number doesn't exist"
        );
        let attributes = self.attributes(number_to_mint).get();

        let uri = self.build_uri(number_to_mint);
        let mut uris = ManagedVec::new();
        uris.push(uri);

        self.send().esdt_nft_create::<NftAttributes<Self::Api>>(
            &token,
            &BigUint::from(1u64),
            &name,
            &royalties,
            &ManagedBuffer::new(),
            &attributes,
            &uris,
        );
    }

    #[only_owner]
    #[endpoint(createNftEnum)]
    fn create_nft_enum_attributs(&self, number_to_mint: u64) {
        let token = self.nft_token_id().get();
        let royalties = BigUint::from(ROYALTIES);
        let name = self.name().get();

        require!(
            !self.attributes_enum(number_to_mint).is_empty(),
            "On-chain attributes for this number doesn't exist"
        );
        let attributes = self.attributes_enum(number_to_mint).get();

        let uri = self.build_uri(number_to_mint);
        let mut uris = ManagedVec::new();
        uris.push(uri);

        self.send().esdt_nft_create::<NftAttributesEnum>(
            &token,
            &BigUint::from(1u64),
            &name,
            &royalties,
            &ManagedBuffer::new(),
            &attributes,
            &uris,
        );
    }

    fn build_uri(&self, number_to_mint: u64) -> ManagedBuffer {
        let mut uri = ManagedBuffer::new_from_bytes(IPFS_GATEWAY);
        let cid = self.image_cid().get();
        let slash = ManagedBuffer::from("/".as_bytes());
        let index_file = ManagedBuffer::new_from_bytes(number_to_mint.to_string().as_bytes());
        let uri_extension = ManagedBuffer::from(".jpg".as_bytes());

        uri.append(&cid);
        uri.append(&slash);
        uri.append(&index_file);
        uri.append(&uri_extension);

        uri
    }

    #[only_owner]
    #[endpoint(fillAttributes)]
    fn fill_attributes_endpoint(
        &self,
        #[var_args] attributes: MultiValueEncoded<AttributesAsMultiValue<Self::Api>>,
    ) {
        for attribut in attributes.into_iter() {
            let (number, background, skin, hat, accessories) = attribut.into_tuple();
            let attributes = NftAttributes {
                background: background,
                skin: skin,
                hat: hat,
                accessories: accessories,
            };
            self.attributes(number).set(&attributes);
        }
    }

    #[only_owner]
    #[endpoint(fillAttributesEnum)]
    fn fill_attributes_endpoint_enum(
        &self,
        #[var_args] attributes: MultiValueEncoded<AttributesAsMultiValueEnum>,
    ) {
        for attribut in attributes.into_iter() {
            let (number, background, skin, hat, accessories) = attribut.into_tuple();
            let attributes = NftAttributesEnum {
                background: background,
                skin: skin,
                hat: hat,
                accessories: accessories,
            };
            self.attributes_enum(number).set(&attributes);
        }
    }

    #[storage_mapper("attributes")]
    fn attributes(&self, number: u64) -> SingleValueMapper<NftAttributes<Self::Api>>;

    #[storage_mapper("attributes")]
    fn attributes_enum(&self, number: u64) -> SingleValueMapper<NftAttributesEnum>;

    #[storage_mapper("name")]
    fn name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("imageCid")]
    fn image_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("metadataCid")]
    fn metadata_cid(&self) -> SingleValueMapper<ManagedBuffer>;
}

#![no_std]
extern crate alloc;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const IPFS_GATEWAY: &[u8] = "https://ipfs.io/ipfs/".as_bytes();
const ROYALTIES: u64 = 500;

mod token;
mod attribut;
use attribut::NftAttributes;
use alloc::string::ToString;

#[elrond_wasm::contract]
pub trait NftOnChain: token::TokenModule  {
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
    #[endpoint(createNft)]
    fn create_nft(&self, number_to_mint: u64) {
        let token = self.nft_token_id().get();
        let royalties = BigUint::from(ROYALTIES);
        let name = self.name().get();

        require!(
            !self.attributes(number_to_mint).is_empty(),
            "On-chain attributes for this number doesn't exit"
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
            &uris
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
        #[var_args] attributes: MultiValueEncoded<MultiValue5<u64, ManagedBuffer, ManagedBuffer, ManagedBuffer, Option<ManagedBuffer>>>
    ) {
        for attribut in attributes.into_iter() {
            let (number, background, skin, hat, accessories) = attribut.into_tuple();
            let attributes = NftAttributes {
                background: background,
                skin: skin,
                hat: hat,
                accessories: accessories
            };
            self.attributes(number).set(&attributes);
        }
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

use scrypto::types::*;
use scrypto::utils::*;

/// An address allocator defines how new addresses and identifiers
/// are generated.
pub struct AddressAllocator {
    count: u32,
}

impl AddressAllocator {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    pub fn new_package_address(&mut self, tx_hash: H256) -> Address {
        let mut data = tx_hash.as_ref().to_vec();
        data.extend(self.count.to_le_bytes());
        self.count += 1;

        let hash = sha256_twice(data);
        Address::Package(hash.lower_26_bytes())
    }

    pub fn new_component_address(&mut self, tx_hash: H256) -> Address {
        let mut data = tx_hash.as_ref().to_vec();
        data.extend(self.count.to_le_bytes());
        self.count += 1;

        let hash = sha256_twice(data);
        Address::Component(hash.lower_26_bytes())
    }

    pub fn new_resource_address(&mut self, tx_hash: H256) -> Address {
        let mut data = tx_hash.as_ref().to_vec();
        data.extend(self.count.to_le_bytes());
        self.count += 1;

        let hash = sha256_twice(data);
        Address::ResourceDef(hash.lower_26_bytes())
    }

    pub fn new_bucket_id(&mut self) -> BID {
        self.count += 1;
        BID(self.count - 1)
    }

    pub fn new_vault_id(&mut self, tx_hash: H256) -> VID {
        self.count += 1;
        VID(tx_hash, self.count - 1)
    }

    pub fn new_rid(&mut self) -> RID {
        self.count += 1;
        RID(self.count)
    }

    pub fn new_mid(&mut self, tx_hash: H256) -> MID {
        self.count += 1;
        MID(tx_hash, self.count - 1)
    }
}

impl Default for AddressAllocator {
    fn default() -> Self {
        Self::new()
    }
}

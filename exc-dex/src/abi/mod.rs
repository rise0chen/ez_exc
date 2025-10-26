use alloy::primitives::{Address, FixedBytes, I256, U160, U256};
use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "src/abi/ERC20.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Cex,
    "src/abi/Cex.json"
);

impl Cex::Pool {
    pub fn new(kind: u8, address: Address, fee: u16) -> Self {
        let kind = U256::from(kind).saturating_shl(8);
        let address = U256::from(U160::from_be_bytes(address.0 .0)).saturating_shl(16);
        let fee = U256::from(fee).saturating_shl(176);
        Self::from_underlying((kind | address | fee).into())
    }
    pub fn zero_for_one(self, zero_for_one: bool) -> Self {
        let d = if zero_for_one {
            let mask = FixedBytes(I256::unchecked_from(1).to_be_bytes());
            self.clone().into_underlying().bit_or(mask)
        } else {
            let mask = FixedBytes(I256::unchecked_from(-256).to_be_bytes());
            self.clone().into_underlying().bit_and(mask)
        };
        Self::from_underlying(d)
    }
}

impl Cex::Place {
    pub fn new(price: U160, amount: i128) -> Self {
        let price = U256::from(price).saturating_shl(96);
        let amount = U256::from_limbs(I256::unchecked_from(amount).into_limbs()) & U256::MAX.wrapping_shl(96).not();
        Self::from_underlying((price | amount).into())
    }
}

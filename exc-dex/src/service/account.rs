use super::Dex;
use crate::abi::ERC20;
use crate::error::map_err;
use alloy::eips::BlockId;
use alloy::primitives::utils::format_units;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::account::Position;

impl Dex {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        let quote = ERC20::new(self.quote, &self.rpc);
        let quote_decimals = quote.decimals().call().await.map_err(map_err)?;
        let balance = quote.balanceOf(self.vault);
        let balance = balance.block(BlockId::pending()).call().await.map_err(map_err)?;
        let balance = format_units(balance, quote_decimals).unwrap();
        Ok(balance.parse().unwrap())
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<Position, ExchangeError> {
        let token = ERC20::new(symbol.base_id.parse().unwrap(), &self.rpc);
        let balance = token.balanceOf(self.vault);
        let balance = balance.block(BlockId::pending()).call().await.map_err(map_err)?;
        let balance = format_units(balance, symbol.precision as u8).unwrap();
        Ok(Position::new(balance.parse().unwrap()))
    }
}

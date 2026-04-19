use super::Dex;
use crate::abi::ERC20;
use crate::error::map_err;
use alloy::providers::Provider;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;

impl Dex {
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        let gas = self.rpc.get_gas_price().await.map_err(|e| map_err(e.into()))? as u64;
        if self.key.gas_price < gas {
            self.key.gas_price = gas;
            tracing::info!("dex gas price from {} to {}", self.key.gas_price, self.key.gas_price);
        }
        let chain_id = self.rpc.get_chain_id().await.map_err(|e| map_err(e.into()))?;
        let chain_info = crate::three::chain::get_chain(chain_id).await.unwrap();
        let native_token = chain_info.native_currency;
        let token_info = crate::three::token::get_token(&native_token.symbol).await;
        if let Ok(Some(info)) = token_info {
            let p = info.current_price;
            symbol.fee_coin = p / 10.0f64.powi(native_token.decimals);
            symbol.fee = (self.key.gas_limit * self.key.gas_price) as f64 * symbol.fee_coin / symbol.min_usd;
            tracing::info!("dex {} fee: price {p}, rate {}", chain_info.chain, symbol.fee);
        } else {
            tracing::error!("dex {} fee {} price failed: {:?}", chain_info.chain, native_token.symbol, token_info);
        };

        let base = ERC20::new(symbol.base_id.parse().unwrap(), &self.rpc);
        if symbol.quote_id.is_empty() {
            symbol.quote_id = self.quote.to_string();
        }
        let quote = ERC20::new(symbol.quote_id.parse().unwrap(), &self.rpc);
        let base_decimals = base.decimals().call().await.map_err(map_err)? as i8;
        let quote_decimals = quote.decimals().call().await.map_err(map_err)? as i8;
        if symbol.multi_price != 1.0 {
            tracing::info!("dex multi_price from {} to {}", symbol.multi_price, 1.0);
            symbol.multi_price = 1.0;
        }
        if symbol.multi_size != 1.0 {
            tracing::info!("dex multi_size from {} to {}", symbol.multi_size, 1.0);
            symbol.multi_size = 1.0;
        }
        if symbol.precision != base_decimals {
            tracing::info!("dex precision_size from {} to {}", symbol.precision, base_decimals);
            symbol.precision = base_decimals;
        }
        if symbol.precision_price != quote_decimals {
            tracing::info!("dex precision_price from {} to {}", symbol.precision_price, quote_decimals);
            symbol.precision_price = quote_decimals;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, _symbol: &Symbol) -> Result<f64, ExchangeError> {
        Ok(0.0)
    }

    pub async fn get_funding_rate(&mut self, _symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        Ok(FundingRate::default())
    }
    pub async fn get_funding_rate_history(&mut self, _symbol: &Symbol, _day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        Ok(Vec::new())
    }
}

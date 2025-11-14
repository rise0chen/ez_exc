use super::Dydx;
use bigdecimal::ToPrimitive;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::OffsetDateTime;

impl Dydx {
    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let resp = self.indexer.markets().get_perpetual_market(&symbol_id).await?;
        let interval: u64 = 60 * 60 * 1000;
        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        Ok(FundingRate {
            rate: resp.next_funding_rate.to_f64().unwrap(),
            time: ((now / interval) + 1) * interval,
            interval,
        })
    }
}

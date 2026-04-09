use super::Dex;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;

impl Dex {
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

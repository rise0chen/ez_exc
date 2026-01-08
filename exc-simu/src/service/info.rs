use super::Simu;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;

impl Simu {
    pub async fn get_funding_rate(&mut self, _symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        Ok(FundingRate::default())
    }
    pub async fn get_funding_rate_history(&mut self, _symbol: &Symbol, _day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        Ok(Vec::new())
    }
}

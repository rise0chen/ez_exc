use super::Simu;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;

impl Simu {
    pub async fn get_funding_rate(&mut self, _symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        Ok(FundingRate::default())
    }
}

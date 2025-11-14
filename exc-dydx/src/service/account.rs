use super::Dydx;
use bigdecimal::ToPrimitive;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;

impl Dydx {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        let account = self.wallet.account(0, &mut self.client).await?;
        let subaccount = account.subaccount(0)?;
        let resp = self.indexer.accounts().get_subaccount(&subaccount).await?;
        Ok(resp.equity.to_f64().unwrap())
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let account = self.wallet.account(0, &mut self.client).await?;
        let subaccount = account.subaccount(0)?;
        let resp = self.indexer.accounts().get_subaccount_perpetual_positions(&subaccount, None).await?;
        let position = resp
            .into_iter()
            .filter_map(|x| if x.market == symbol_id { Some(x.size.to_f64().unwrap()) } else { None })
            .sum();
        Ok(position)
    }
}

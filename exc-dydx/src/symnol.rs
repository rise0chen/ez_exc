use dydx::indexer::Ticker;
use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> Ticker {
    let symbol = if symbol.is_spot() {
        format!("{}-{}", symbol.base, symbol.quote)
    } else {
        format!("{}{}{}-{}", symbol.prefix, symbol.base, symbol.suffix, symbol.quote)
    };
    Ticker(symbol)
}

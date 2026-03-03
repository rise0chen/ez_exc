use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    format!("{}_{}", symbol.base.to_lowercase(), symbol.quote.to_lowercase())
}

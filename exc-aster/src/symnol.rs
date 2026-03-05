use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    format!("{}{}", symbol.base.to_lowercase(), symbol.quote.to_lowercase())
}

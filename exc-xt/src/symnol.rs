use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    format!("{}{}{}_{}", symbol.prefix, symbol.base, symbol.suffix, symbol.quote).to_lowercase()
}

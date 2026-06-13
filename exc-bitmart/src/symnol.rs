use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    if symbol.is_spot() {
        format!("{}{}{}_{}", symbol.prefix, symbol.base, symbol.suffix, symbol.quote)
    } else {
        format!("{}{}{}{}", symbol.prefix, symbol.base, symbol.suffix, symbol.quote)
    }
}

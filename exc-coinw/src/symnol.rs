use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    if symbol.quote == "USDT" {
        format!("{}{}{}", symbol.prefix, symbol.base, symbol.suffix)
    } else {
        format!("{}{}{}{}", symbol.prefix, symbol.base, symbol.suffix, symbol.quote)
    }
}

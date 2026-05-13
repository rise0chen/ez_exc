use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    if symbol.quote == "USDT" {
        symbol.base.to_string()
    } else {
        format!("{}{}", symbol.base, symbol.quote)
    }
}

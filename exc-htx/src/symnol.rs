use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    if symbol.is_spot() {
        let mut symbol = format!("{}{}", symbol.base, symbol.quote);
        symbol.make_ascii_lowercase();
        symbol
    } else {
        format!("{}{}{}-{}", symbol.prefix, symbol.base, symbol.suffix, symbol.quote)
    }
}

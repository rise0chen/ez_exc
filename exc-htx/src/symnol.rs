use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    let mut symbol = if symbol.is_spot() {
        format!("{}{}", symbol.base, symbol.quote)
    } else {
        format!("{}{}{}-{}", symbol.prefix, symbol.base, symbol.suffix, symbol.quote)
    };
    symbol.make_ascii_lowercase();
    symbol
}

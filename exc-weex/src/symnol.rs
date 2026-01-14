use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    if symbol.is_spot() {
        format!("{}{}_SPBL", symbol.base, symbol.quote)
    } else {
        format!(
            "cmt_{}{}{}{}",
            symbol.prefix,
            symbol.base.to_lowercase(),
            symbol.suffix,
            symbol.quote.to_lowercase()
        )
    }
}

use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> i16 {
    symbol.base_id.parse().unwrap()
}

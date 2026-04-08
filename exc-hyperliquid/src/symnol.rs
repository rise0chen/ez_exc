use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    let mut symbol = symbol.base.split(':');
    match (symbol.next(), symbol.next()) {
        (Some(s), None) => s.to_ascii_uppercase(),
        (Some(dex), Some(s)) => format!("{}:{}", dex.to_ascii_lowercase(), s.to_ascii_uppercase()),
        _ => panic!("invaild symbol: {:?}", symbol),
    }
}

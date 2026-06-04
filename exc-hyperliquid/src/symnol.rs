use exc_util::symbol::Symbol;

pub fn symbol_id(symbol: &Symbol) -> String {
    if symbol.is_spot() {
        let id: u64 = symbol.base_id.parse().unwrap();
        return format!("@{}", id - 10000);
    }
    let mut s = symbol.base.split(':');
    match (s.next(), s.next()) {
        (Some(s), None) => format!("{}{}{}", symbol.prefix, s, symbol.suffix),
        (Some(dex), Some(s)) => format!("{}:{}{}{}", dex, symbol.prefix, s, symbol.suffix),
        _ => panic!("invaild symbol: {:?}", s),
    }
}

pub fn dex_symbol(symbol: &Symbol) -> (Option<String>, String) {
    if symbol.is_spot() {
        return (None, symbol.base.to_uppercase());
    }
    let mut symbol = symbol.base.split(':');
    match (symbol.next(), symbol.next()) {
        (Some(symbol), None) => (None, symbol.to_uppercase()),
        (Some(dex), Some(symbol)) => (Some(dex.to_ascii_lowercase()), symbol.to_uppercase()),
        _ => panic!("invaild symbol: {:?}", symbol),
    }
}

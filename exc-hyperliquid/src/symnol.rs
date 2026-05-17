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

pub fn dex(symbol: &Symbol) -> Option<String> {
    if symbol.is_spot() {
        return None;
    }
    let mut symbol = symbol.base.split(':');
    match (symbol.next(), symbol.next()) {
        (Some(_), None) => None,
        (Some(dex), Some(_)) => Some(dex.to_ascii_lowercase()),
        _ => panic!("invaild symbol: {:?}", symbol),
    }
}

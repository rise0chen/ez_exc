use exc_util::symbol::{Symbol, SymbolKind};

pub fn symbol_id(symbol: &Symbol) -> String {
    match symbol.kind {
        SymbolKind::Spot => format!("{}{}{}-{}", symbol.prefix, symbol.base, symbol.suffix, symbol.quote),
        SymbolKind::Linear => format!("{}{}{}-{}-SWAP", symbol.prefix, symbol.base, symbol.suffix, symbol.quote),
        SymbolKind::Option => format!("{}{}{}-{}", symbol.prefix, symbol.base, symbol.suffix, symbol.quote),
        _ => todo!(),
    }
}

pub use exc_core::Asset;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum SymbolKind {
    Spot,
    Derivative,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub base: Asset,
    pub base_id: String,
    pub quote: Asset,
    pub quote_id: String,
    pub prefix: String,
    pub suffix: String,
}
impl Symbol {
    pub fn spot(base: Asset, quote: Asset) -> Self {
        Self {
            kind: SymbolKind::Spot,
            base,
            base_id: String::new(),
            quote,
            quote_id: String::new(),
            prefix: String::new(),
            suffix: String::new(),
        }
    }
    pub fn derivative(base: Asset, quote: Asset) -> Self {
        Self {
            kind: SymbolKind::Derivative,
            base,
            base_id: String::new(),
            quote,
            quote_id: String::new(),
            prefix: String::new(),
            suffix: String::new(),
        }
    }
    pub fn is_spot(&self) -> bool {
        matches!(self.kind, SymbolKind::Spot)
    }
    pub fn is_derivative(&self) -> bool {
        matches!(self.kind, SymbolKind::Derivative)
    }
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = if self.is_spot() { "S" } else { "F" };
        write!(f, "{}.{}-{}", t, self.base, self.quote)
    }
}

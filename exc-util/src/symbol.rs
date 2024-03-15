pub use exc_core::Asset;

#[derive(Debug, Clone, Copy)]
pub enum SymbolKind {
    Spot,
    Derivative,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub base: Asset,
    pub quote: Asset,
    pub prefix: String,
    pub suffix: String,
}
impl Symbol {
    pub fn spot(base: Asset, quote: Asset) -> Self {
        Self {
            kind: SymbolKind::Spot,
            base,
            quote,
            prefix: String::new(),
            suffix: String::new(),
        }
    }
    pub fn derivative(base: Asset, quote: Asset) -> Self {
        Self {
            kind: SymbolKind::Derivative,
            base,
            quote,
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

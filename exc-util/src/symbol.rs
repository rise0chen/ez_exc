pub use exc_core::Asset;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    /// 未知
    Unknown,
    /// 现货
    Spot,
    /// 正向永续
    Linear,
    /// 反向永续
    Inverse,
    /// 交割期权
    Option,
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
    /// 一张合约代表多少Token
    pub multi_size: f64,
    /// 数量精度
    pub precision: i8,
}
impl Symbol {
    pub fn unknown(base: Asset, quote: Asset) -> Self {
        Self {
            kind: SymbolKind::Unknown,
            base,
            base_id: String::new(),
            quote,
            quote_id: String::new(),
            prefix: String::new(),
            suffix: String::new(),
            multi_size: 1.0,
            precision: 0,
        }
    }
    pub fn spot(base: Asset, quote: Asset) -> Self {
        Self {
            kind: SymbolKind::Spot,
            base,
            base_id: String::new(),
            quote,
            quote_id: String::new(),
            prefix: String::new(),
            suffix: String::new(),
            multi_size: 1.0,
            precision: 0,
        }
    }
    pub fn derivative(base: Asset, quote: Asset) -> Self {
        Self {
            kind: SymbolKind::Linear,
            base,
            base_id: String::new(),
            quote,
            quote_id: String::new(),
            prefix: String::new(),
            suffix: String::new(),
            multi_size: 1.0,
            precision: 0,
        }
    }
    pub fn is_spot(&self) -> bool {
        matches!(self.kind, SymbolKind::Spot)
    }
    pub fn is_derivative(&self) -> bool {
        matches!(self.kind, SymbolKind::Linear)
    }
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match &self.kind {
            SymbolKind::Unknown => "U",
            SymbolKind::Spot => "S",
            SymbolKind::Linear => "F",
            SymbolKind::Inverse => "-F",
            SymbolKind::Option => "Q",
        };
        write!(f, "{}.{}-{}", t, self.base, self.quote)
    }
}

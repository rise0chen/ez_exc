pub use crate::asset::Asset;
use crate::types::book::Order;
use rust_decimal::Decimal;
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
    /// 一张合约价格代表多少Token
    pub multi_price: f64,
    /// 一张合约代表多少Token
    pub multi_size: f64,
    /// 数量精度
    pub precision: i8,
    /// 价格精度
    pub precision_price: i8,
    /// 最小下单金额
    pub min_usd: f64,
    /// 最小下单数量
    pub min_size: f64,
    /// 吃单手续费率
    pub fee: f64,
    /// 手续费币种价格
    pub fee_coin: f64,
    /// 是否可以开仓
    pub can_open: bool,
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
            multi_price: 1.0,
            multi_size: 1.0,
            precision: 0,
            precision_price: 2,
            min_usd: 5.0,
            min_size: 0.0,
            fee: 0.0,
            fee_coin: 1.0,
            can_open: true,
        }
    }
    pub fn spot(base: Asset, quote: Asset) -> Self {
        let mut ret = Self::unknown(base, quote);
        ret.kind = SymbolKind::Spot;
        ret
    }
    pub fn derivative(base: Asset, quote: Asset) -> Self {
        let mut ret = Self::unknown(base, quote);
        ret.kind = SymbolKind::Linear;
        ret
    }
    pub fn option(base: Asset, quote: Asset) -> Self {
        let mut ret = Self::unknown(base, quote);
        ret.kind = SymbolKind::Option;
        ret
    }

    pub fn is_spot(&self) -> bool {
        matches!(self.kind, SymbolKind::Spot)
    }
    pub fn is_derivative(&self) -> bool {
        matches!(self.kind, SymbolKind::Linear)
    }
}
impl Symbol {
    pub fn contract_size(&self, token_size: f64) -> Decimal {
        let multi_price = self.multi_price;
        let multi_size = self.multi_size;
        let size = token_size / multi_price / multi_size;
        let precision = self.precision;
        let is_spot = self.is_spot();
        match precision {
            0.. => {
                let r = Decimal::from_f64_retain(size).unwrap();
                if is_spot {
                    r.trunc_with_scale(precision as u32)
                } else {
                    r.round_dp(precision as u32)
                }
            }
            ..0 => {
                let p = 10f64.powi(-precision as i32);
                let int = if is_spot {
                    (size / p).trunc() as i64 * p as i64
                } else {
                    (size / p).round() as i64 * p as i64
                };
                Decimal::new(int, 0)
            }
        }
    }
    pub fn token_size(&self, contract_size: f64) -> f64 {
        let multi_price = self.multi_price;
        let multi_size = self.multi_size;
        contract_size * multi_price * multi_size
    }
    pub fn contract_price(&self, token_price: f64, buy: bool) -> Decimal {
        let multi_price = self.multi_price;
        let price = token_price * multi_price;
        let precision = self.precision_price;
        match precision {
            0.. => {
                let r = Decimal::from_f64_retain(price).unwrap();
                let r = r.trunc_with_scale(precision as u32);
                if buy {
                    r + Decimal::new(1, precision as u32)
                } else {
                    r
                }
            }
            ..0 => {
                let p = 10f64.powi(-precision as i32);
                let int = if buy {
                    (price / p).ceil() as i64 * p as i64
                } else {
                    (price / p).trunc() as i64 * p as i64
                };
                Decimal::new(int, 0)
            }
        }
    }
    pub fn token_price(&self, contract_price: f64) -> f64 {
        let multi_price = self.multi_price;
        contract_price / multi_price
    }

    pub fn min_once(&self, price: f64) -> f64 {
        let one_size = self.token_size(1.0 / 10f64.powi(self.precision as i32));
        let one_price = self.token_price(1.0 / 10f64.powi(self.precision_price as i32));
        let min_by_usd = self.min_usd / (price - one_price);
        let min_by_size = self.token_size(self.min_size);
        one_size.max(min_by_size).max(min_by_usd)
    }

    pub fn order(&self, p: f64, s: f64) -> Order {
        Order::new(self.token_price(p), self.token_size(s))
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

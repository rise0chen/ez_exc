use crate::symbol::Symbol;

#[derive(Debug, Default, Clone)]
pub struct StRate {
    pub rate: f64,
    pub start_time: u64,
    pub apy: f64,
}

pub fn common_st_rate(symbol: &Symbol) -> Option<StRate> {
    let rate = match symbol.base.as_str() {
        "XAUT" => StRate {
            rate: 0.998,
            start_time: 0,
            apy: 0.0,
        },
        "SLVON" => StRate {
            rate: 0.905,
            start_time: 1776412800000,
            apy: -0.005,
        },
        "IAU" => StRate {
            rate: 0.01888,
            start_time: 1776412800000,
            apy: -0.0025,
        },
        _ => return None,
    };
    Some(rate)
}

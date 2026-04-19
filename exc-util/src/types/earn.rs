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
            rate: 0.9975,
            start_time: 0,
            apy: 0.0,
        },
        "XAUT0" => StRate {
            rate: 0.9975,
            start_time: 0,
            apy: 0.0,
        },
        "PAXG" => StRate {
            rate: 0.99875,
            start_time: 0,
            apy: 0.0,
        },
        "IAU" => StRate {
            rate: 0.01886,
            start_time: 1776412800000,
            apy: -0.0025,
        },
        "SLVON" => StRate {
            rate: 0.905,
            start_time: 1776412800000,
            apy: -0.005,
        },
        _ => return None,
    };
    Some(rate)
}

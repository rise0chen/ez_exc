#[derive(Debug, Default, Clone)]
pub struct StRate {
    pub rate: f64,
    pub start_time: u64,
    pub apy: f64,
}

pub fn common_st_rate(symbol: &str) -> Option<StRate> {
    let rate = match symbol.to_uppercase().as_str() {
        "CL" | "WTI" | "XTI" | "USOIL" => StRate {
            rate: 0.952,
            start_time: 0,
            apy: 0.0,
        },
        "USO" => StRate {
            rate: 1.396,
            start_time: 1776412800000,
            apy: -0.0152,
        },
        "XAUT" | "XAUT0" => StRate {
            rate: 0.9975,
            start_time: 0,
            apy: 0.0,
        },
        "PAXG" => StRate {
            rate: 0.99875,
            start_time: 0,
            apy: 0.0,
        },
        "IAU" | "IAUON" => StRate {
            rate: 0.01878,
            start_time: 1776412800000,
            apy: -0.0025,
        },
        "SLV" | "SLVON" => StRate {
            rate: 0.905,
            start_time: 1776412800000,
            apy: -0.005,
        },
        _ => return None,
    };
    Some(rate)
}

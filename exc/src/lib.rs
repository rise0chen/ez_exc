pub use exc_custom as custom;
pub use exc_simu as simu;
pub use exc_util as util;

use exc_aden::{key::Key as AdenKey, service::Aden};
use exc_aster::{key::Key as AsterKey, service::Aster};
use exc_binance::{key::Key as BinanceKey, service::Binance};
use exc_bitget::{key::Key as BitgetKey, service::Bitget};
use exc_bitmart::{key::Key as BitmartKey, service::Bitmart};
use exc_bitmex::{key::Key as BitmexKey, service::Bitmex};
use exc_bitunix::{key::Key as BitunixKey, service::Bitunix};
use exc_bybit::{key::Key as BybitKey, service::Bybit};
use exc_coinw::{key::Key as CoinwKey, service::Coinw};
use exc_custom::service::Custom;
use exc_dex::{key::Key as DexKey, service::Dex};
use exc_dydx::{key::Key as DydxKey, service::Dydx};
use exc_gate::{key::Key as GateKey, service::Gate};
use exc_grvt::{key::Key as GrvtKey, service::Grvt};
use exc_htx::{key::Key as HtxKey, service::Htx};
use exc_hyperliquid::{key::Key as HyperliquidKey, service::Hyperliquid};
use exc_kcex::{key::Key as KcexKey, service::Kcex};
use exc_lbank::{key::Key as LbankKey, service::Lbank};
use exc_lighter::{key::Key as LighterKey, service::Lighter};
use exc_mexc::{key::Key as MexcKey, service::Mexc};
use exc_okx::{key::Key as OkxKey, service::Okx};
use exc_paradex::{key::Key as ParadexKey, service::Paradex};
use exc_simu::service::Simu;
use exc_toobit::{key::Key as ToobitKey, service::Toobit};
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::account::{Balance, Position};
use exc_util::types::book::Depth;
use exc_util::types::earn::StRate;
use exc_util::types::info::FundingRate;
use exc_util::types::order::{Order, OrderId, PlaceOrderRequest};
use exc_weex::{key::Key as WeexKey, service::Weex};
use exc_xt::{key::Key as XtKey, service::Xt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExchangeConfig {
    Xt {
        #[serde(flatten)]
        key: XtKey,
    },
    Mexc {
        #[serde(flatten)]
        key: MexcKey,
    },
    Lighter {
        #[serde(flatten)]
        key: LighterKey,
    },
    Lbank {
        #[serde(flatten)]
        key: LbankKey,
    },
    Kcex {
        #[serde(flatten)]
        key: KcexKey,
    },
    Okx {
        #[serde(flatten)]
        key: OkxKey,
    },
    Paradex {
        #[serde(flatten)]
        key: ParadexKey,
    },
    Weex {
        #[serde(flatten)]
        key: WeexKey,
    },
    Gate {
        #[serde(flatten)]
        key: GateKey,
    },
    Grvt {
        #[serde(flatten)]
        key: GrvtKey,
    },
    Htx {
        #[serde(flatten)]
        key: HtxKey,
    },
    Hyperliquid {
        #[serde(flatten)]
        key: HyperliquidKey,
    },
    Bybit {
        #[serde(flatten)]
        key: BybitKey,
    },
    Bitget {
        #[serde(flatten)]
        key: BitgetKey,
    },
    Bitmart {
        #[serde(flatten)]
        key: BitmartKey,
    },
    Bitmex {
        #[serde(flatten)]
        key: BitmexKey,
    },
    Bitunix {
        #[serde(flatten)]
        key: BitunixKey,
    },
    Binance {
        #[serde(flatten)]
        key: BinanceKey,
    },
    Aster {
        #[serde(flatten)]
        key: AsterKey,
    },
    Aden {
        #[serde(flatten)]
        key: AdenKey,
    },
    Dex {
        #[serde(flatten)]
        key: DexKey,
    },
    Dydx {
        #[serde(flatten)]
        key: DydxKey,
    },
    Coinw {
        #[serde(flatten)]
        key: CoinwKey,
    },
    Toobit {
        #[serde(flatten)]
        key: ToobitKey,
    },
    Simu {
        path: String,
        interval: u64,
    },
    #[default]
    None,
}

#[derive(Clone)]
pub enum Exchange {
    Xt(Xt),
    Mexc(Mexc),
    Lighter(Lighter),
    Lbank(Lbank),
    Kcex(Kcex),
    Okx(Okx),
    Paradex(Paradex),
    Weex(Weex),
    Gate(Gate),
    Grvt(Grvt),
    Htx(Htx),
    Hyperliquid(Hyperliquid),
    Bybit(Bybit),
    Bitget(Bitget),
    Bitmart(Bitmart),
    Bitmex(Bitmex),
    Bitunix(Bitunix),
    Binance(Binance),
    Aster(Aster),
    Aden(Aden),
    Dex(Dex),
    Dydx(Dydx),
    Coinw(Coinw),
    Toobit(Toobit),
    Custom(Custom),
    Simu(Simu),
    None,
}

impl Exchange {
    pub fn new(cfg: ExchangeConfig) -> Self {
        match cfg {
            ExchangeConfig::Xt { key } => Self::Xt(Xt::new(key)),
            ExchangeConfig::Mexc { key } => Self::Mexc(Mexc::new(key)),
            ExchangeConfig::Lighter { key } => {
                let exc = Lighter::new(key);
                exc.run();
                Self::Lighter(exc)
            }
            ExchangeConfig::Lbank { key } => {
                let exc = Lbank::new(key);
                exc.run();
                Self::Lbank(exc)
            }
            ExchangeConfig::Kcex { key } => Self::Kcex(Kcex::new(key)),
            ExchangeConfig::Okx { key } => Self::Okx(Okx::new(key)),
            ExchangeConfig::Paradex { key } => Self::Paradex(tokio::task::block_in_place(move || {
                tokio::runtime::Handle::current().block_on(Paradex::new(key))
            })),
            ExchangeConfig::Weex { key } => Self::Weex(Weex::new(key)),
            ExchangeConfig::Gate { key } => {
                let exc = Gate::new(key);
                exc.run();
                Self::Gate(exc)
            }
            ExchangeConfig::Grvt { key } => Self::Grvt(tokio::task::block_in_place(move || {
                tokio::runtime::Handle::current().block_on(Grvt::new(key))
            })),
            ExchangeConfig::Htx { key } => {
                let exc = Htx::new(key);
                exc.run();
                Self::Htx(exc)
            }
            ExchangeConfig::Hyperliquid { key } => {
                let exc = Hyperliquid::new(key);
                exc.run();
                Self::Hyperliquid(exc)
            }
            ExchangeConfig::Bybit { key } => Self::Bybit(Bybit::new(key)),
            ExchangeConfig::Bitget { key } => Self::Bitget(Bitget::new(key)),
            ExchangeConfig::Bitmart { key } => {
                let exc = Bitmart::new(key);
                exc.run();
                Self::Bitmart(exc)
            }
            ExchangeConfig::Bitmex { key } => {
                let exc = Bitmex::new(key);
                exc.run();
                Self::Bitmex(exc)
            }
            ExchangeConfig::Bitunix { key } => {
                let exc = Bitunix::new(key);
                exc.run();
                Self::Bitunix(exc)
            }
            ExchangeConfig::Binance { key } => Self::Binance(Binance::new(key)),
            ExchangeConfig::Aster { key } => Self::Aster(Aster::new(key)),
            ExchangeConfig::Aden { key } => Self::Aden(Aden::new(key)),
            ExchangeConfig::Dex { key } => Self::Dex(tokio::task::block_in_place(move || {
                tokio::runtime::Handle::current().block_on(Dex::new(key))
            })),
            ExchangeConfig::Dydx { key } => Self::Dydx(tokio::task::block_in_place(move || {
                tokio::runtime::Handle::current().block_on(Dydx::new(key))
            })),
            ExchangeConfig::Coinw { key } => {
                let exc = Coinw::new(key);
                exc.run();
                Self::Coinw(exc)
            }
            ExchangeConfig::Toobit { key } => Self::Toobit(Toobit::new(key)),
            ExchangeConfig::Simu { path, interval } => {
                let data = std::fs::read(path).unwrap();
                let prices = serde_json::from_slice(&data).unwrap();
                Self::Simu(Simu::new(prices, interval))
            }
            ExchangeConfig::None => Self::None,
        }
    }
}

impl Exchange {
    pub async fn get_balance(&mut self) -> Result<Balance, ExchangeError> {
        match self {
            Exchange::Xt(e) => e.get_balance().await,
            Exchange::Mexc(e) => e.get_balance().await,
            Exchange::Lighter(e) => e.get_balance().await,
            Exchange::Lbank(e) => e.get_balance().await,
            Exchange::Kcex(e) => e.get_balance().await,
            Exchange::Okx(e) => e.get_balance().await,
            Exchange::Paradex(e) => e.get_balance().await,
            Exchange::Weex(e) => e.get_balance().await,
            Exchange::Gate(e) => e.get_balance().await,
            Exchange::Grvt(e) => e.get_balance().await,
            Exchange::Htx(e) => e.get_balance().await,
            Exchange::Hyperliquid(e) => e.get_balance().await,
            Exchange::Bybit(e) => e.get_balance().await,
            Exchange::Bitget(e) => e.get_balance().await,
            Exchange::Bitmart(e) => e.get_balance().await,
            Exchange::Bitmex(e) => e.get_balance().await,
            Exchange::Bitunix(e) => e.get_balance().await,
            Exchange::Binance(e) => e.get_balance().await,
            Exchange::Aster(e) => e.get_balance().await,
            Exchange::Aden(e) => e.get_balance().await,
            Exchange::Dex(e) => e.get_balance().await,
            Exchange::Dydx(e) => e.get_balance().await,
            Exchange::Coinw(e) => e.get_balance().await,
            Exchange::Toobit(e) => e.get_balance().await,
            Exchange::Custom(_) => todo!(),
            Exchange::Simu(_) => todo!(),
            Exchange::None => todo!(),
        }
    }

    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<Position, ExchangeError> {
        match self {
            Exchange::Xt(e) => e.get_position(symbol).await,
            Exchange::Mexc(e) => e.get_position(symbol).await,
            Exchange::Lighter(e) => e.get_position(symbol).await,
            Exchange::Lbank(e) => e.get_position(symbol).await,
            Exchange::Kcex(e) => e.get_position(symbol).await,
            Exchange::Okx(e) => e.get_position(symbol).await,
            Exchange::Paradex(e) => e.get_position(symbol).await,
            Exchange::Weex(e) => e.get_position(symbol).await,
            Exchange::Gate(e) => e.get_position(symbol).await,
            Exchange::Grvt(e) => e.get_position(symbol).await,
            Exchange::Htx(e) => e.get_position(symbol).await,
            Exchange::Hyperliquid(e) => e.get_position(symbol).await,
            Exchange::Bybit(e) => e.get_position(symbol).await,
            Exchange::Bitget(e) => e.get_position(symbol).await,
            Exchange::Bitmart(e) => e.get_position(symbol).await,
            Exchange::Bitmex(e) => e.get_position(symbol).await,
            Exchange::Bitunix(e) => e.get_position(symbol).await,
            Exchange::Binance(e) => e.get_position(symbol).await,
            Exchange::Aster(e) => e.get_position(symbol).await,
            Exchange::Aden(e) => e.get_position(symbol).await,
            Exchange::Dex(e) => e.get_position(symbol).await,
            Exchange::Dydx(e) => e.get_position(symbol).await,
            Exchange::Coinw(e) => e.get_position(symbol).await,
            Exchange::Toobit(e) => e.get_position(symbol).await,
            Exchange::Custom(_) => todo!(),
            Exchange::Simu(_) => Ok(Position::default()),
            Exchange::None => todo!(),
        }
    }

    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        match self {
            Exchange::Xt(e) => e.perfect_symbol(symbol).await,
            Exchange::Mexc(e) => e.perfect_symbol(symbol).await,
            Exchange::Lighter(e) => e.perfect_symbol(symbol).await,
            Exchange::Lbank(e) => e.perfect_symbol(symbol).await,
            Exchange::Kcex(e) => e.perfect_symbol(symbol).await,
            Exchange::Okx(e) => e.perfect_symbol(symbol).await,
            Exchange::Paradex(e) => e.perfect_symbol(symbol).await,
            Exchange::Weex(e) => e.perfect_symbol(symbol).await,
            Exchange::Gate(e) => e.perfect_symbol(symbol).await,
            Exchange::Grvt(e) => e.perfect_symbol(symbol).await,
            Exchange::Htx(e) => e.perfect_symbol(symbol).await,
            Exchange::Hyperliquid(e) => e.perfect_symbol(symbol).await,
            Exchange::Bybit(e) => e.perfect_symbol(symbol).await,
            Exchange::Bitget(e) => e.perfect_symbol(symbol).await,
            Exchange::Bitmart(e) => e.perfect_symbol(symbol).await,
            Exchange::Bitmex(e) => e.perfect_symbol(symbol).await,
            Exchange::Bitunix(e) => e.perfect_symbol(symbol).await,
            Exchange::Binance(e) => e.perfect_symbol(symbol).await,
            Exchange::Aster(e) => e.perfect_symbol(symbol).await,
            Exchange::Aden(e) => e.perfect_symbol(symbol).await,
            Exchange::Dex(e) => e.perfect_symbol(symbol).await,
            Exchange::Dydx(e) => e.perfect_symbol(symbol).await,
            Exchange::Coinw(e) => e.perfect_symbol(symbol).await,
            Exchange::Toobit(e) => e.perfect_symbol(symbol).await,
            Exchange::Custom(_) => todo!(),
            Exchange::Simu(_) => Ok(()),
            Exchange::None => todo!(),
        }
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        match self {
            Exchange::Xt(e) => e.get_index_price(symbol).await,
            Exchange::Mexc(e) => e.get_index_price(symbol).await,
            Exchange::Lighter(e) => e.get_index_price(symbol).await,
            Exchange::Lbank(e) => e.get_index_price(symbol).await,
            Exchange::Kcex(e) => e.get_index_price(symbol).await,
            Exchange::Okx(e) => e.get_index_price(symbol).await,
            Exchange::Paradex(e) => e.get_index_price(symbol).await,
            Exchange::Weex(e) => e.get_index_price(symbol).await,
            Exchange::Gate(e) => e.get_index_price(symbol).await,
            Exchange::Grvt(e) => e.get_index_price(symbol).await,
            Exchange::Htx(e) => e.get_index_price(symbol).await,
            Exchange::Hyperliquid(e) => e.get_index_price(symbol).await,
            Exchange::Bybit(e) => e.get_index_price(symbol).await,
            Exchange::Bitget(e) => e.get_index_price(symbol).await,
            Exchange::Bitmart(e) => e.get_index_price(symbol).await,
            Exchange::Bitmex(e) => e.get_index_price(symbol).await,
            Exchange::Bitunix(e) => e.get_index_price(symbol).await,
            Exchange::Binance(e) => e.get_index_price(symbol).await,
            Exchange::Aster(e) => e.get_index_price(symbol).await,
            Exchange::Aden(e) => e.get_index_price(symbol).await,
            Exchange::Dex(e) => e.get_index_price(symbol).await,
            Exchange::Dydx(e) => e.get_index_price(symbol).await,
            Exchange::Coinw(e) => e.get_index_price(symbol).await,
            Exchange::Toobit(e) => e.get_index_price(symbol).await,
            Exchange::Custom(e) => e.get_index_price(symbol).await,
            Exchange::Simu(e) => e.get_index_price(symbol).await,
            Exchange::None => todo!(),
        }
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        match self {
            Exchange::Xt(e) => e.get_funding_rate(symbol).await,
            Exchange::Mexc(e) => e.get_funding_rate(symbol).await,
            Exchange::Lighter(e) => e.get_funding_rate(symbol).await,
            Exchange::Lbank(e) => e.get_funding_rate(symbol).await,
            Exchange::Kcex(e) => e.get_funding_rate(symbol).await,
            Exchange::Okx(e) => e.get_funding_rate(symbol).await,
            Exchange::Paradex(e) => e.get_funding_rate(symbol).await,
            Exchange::Weex(e) => e.get_funding_rate(symbol).await,
            Exchange::Gate(e) => e.get_funding_rate(symbol).await,
            Exchange::Grvt(e) => e.get_funding_rate(symbol).await,
            Exchange::Htx(e) => e.get_funding_rate(symbol).await,
            Exchange::Hyperliquid(e) => e.get_funding_rate(symbol).await,
            Exchange::Bybit(e) => e.get_funding_rate(symbol).await,
            Exchange::Bitget(e) => e.get_funding_rate(symbol).await,
            Exchange::Bitmart(e) => e.get_funding_rate(symbol).await,
            Exchange::Bitmex(e) => e.get_funding_rate(symbol).await,
            Exchange::Bitunix(e) => e.get_funding_rate(symbol).await,
            Exchange::Binance(e) => e.get_funding_rate(symbol).await,
            Exchange::Aster(e) => e.get_funding_rate(symbol).await,
            Exchange::Aden(e) => e.get_funding_rate(symbol).await,
            Exchange::Dex(e) => e.get_funding_rate(symbol).await,
            Exchange::Dydx(e) => e.get_funding_rate(symbol).await,
            Exchange::Coinw(e) => e.get_funding_rate(symbol).await,
            Exchange::Toobit(e) => e.get_funding_rate(symbol).await,
            Exchange::Custom(e) => e.get_funding_rate(symbol).await,
            Exchange::Simu(e) => e.get_funding_rate(symbol).await,
            Exchange::None => todo!(),
        }
    }

    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        match self {
            Exchange::Xt(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Mexc(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Lighter(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Lbank(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Kcex(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Okx(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Paradex(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Weex(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Gate(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Grvt(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Htx(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Hyperliquid(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Bybit(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Bitget(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Bitmart(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Bitmex(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Bitunix(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Binance(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Aster(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Aden(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Dex(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Dydx(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Coinw(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Toobit(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Custom(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::Simu(e) => e.get_funding_rate_history(symbol, day).await,
            Exchange::None => todo!(),
        }
    }

    pub async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<StRate, ExchangeError> {
        match self {
            Exchange::Xt(e) => e.get_st_rate(symbol).await,
            Exchange::Mexc(e) => e.get_st_rate(symbol).await,
            Exchange::Lighter(e) => e.get_st_rate(symbol).await,
            Exchange::Lbank(e) => e.get_st_rate(symbol).await,
            Exchange::Kcex(e) => e.get_st_rate(symbol).await,
            Exchange::Okx(e) => e.get_st_rate(symbol).await,
            Exchange::Paradex(e) => e.get_st_rate(symbol).await,
            Exchange::Weex(e) => e.get_st_rate(symbol).await,
            Exchange::Gate(e) => e.get_st_rate(symbol).await,
            Exchange::Grvt(e) => e.get_st_rate(symbol).await,
            Exchange::Htx(e) => e.get_st_rate(symbol).await,
            Exchange::Hyperliquid(e) => e.get_st_rate(symbol).await,
            Exchange::Bybit(e) => e.get_st_rate(symbol).await,
            Exchange::Bitget(e) => e.get_st_rate(symbol).await,
            Exchange::Bitmart(e) => e.get_st_rate(symbol).await,
            Exchange::Bitmex(e) => e.get_st_rate(symbol).await,
            Exchange::Bitunix(e) => e.get_st_rate(symbol).await,
            Exchange::Binance(e) => e.get_st_rate(symbol).await,
            Exchange::Aster(e) => e.get_st_rate(symbol).await,
            Exchange::Aden(e) => e.get_st_rate(symbol).await,
            Exchange::Dex(e) => e.get_st_rate(symbol).await,
            Exchange::Dydx(e) => e.get_st_rate(symbol).await,
            Exchange::Coinw(e) => e.get_st_rate(symbol).await,
            Exchange::Toobit(e) => e.get_st_rate(symbol).await,
            Exchange::Custom(_e) => todo!(),
            Exchange::Simu(_) => todo!(),
            Exchange::None => todo!(),
        }
    }

    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        match self {
            Exchange::Xt(e) => e.get_depth(symbol, limit).await,
            Exchange::Mexc(e) => e.get_depth(symbol, limit).await,
            Exchange::Lighter(e) => e.get_depth(symbol, limit).await,
            Exchange::Lbank(e) => e.get_depth(symbol, limit).await,
            Exchange::Kcex(e) => e.get_depth(symbol, limit).await,
            Exchange::Okx(e) => e.get_depth(symbol, limit).await,
            Exchange::Paradex(e) => e.get_depth(symbol, limit).await,
            Exchange::Weex(e) => e.get_depth(symbol, limit).await,
            Exchange::Gate(e) => e.get_depth(symbol, limit).await,
            Exchange::Grvt(e) => e.get_depth(symbol, limit).await,
            Exchange::Htx(e) => e.get_depth(symbol, limit).await,
            Exchange::Hyperliquid(e) => e.get_depth(symbol, limit).await,
            Exchange::Bybit(e) => e.get_depth(symbol, limit).await,
            Exchange::Bitget(e) => e.get_depth(symbol, limit).await,
            Exchange::Bitmart(e) => e.get_depth(symbol, limit).await,
            Exchange::Bitmex(e) => e.get_depth(symbol, limit).await,
            Exchange::Bitunix(e) => e.get_depth(symbol, limit).await,
            Exchange::Binance(e) => e.get_depth(symbol, limit).await,
            Exchange::Aster(e) => e.get_depth(symbol, limit).await,
            Exchange::Aden(e) => e.get_depth(symbol, limit).await,
            Exchange::Dex(e) => e.get_depth(symbol, limit).await,
            Exchange::Dydx(e) => e.get_depth(symbol, limit).await,
            Exchange::Coinw(e) => e.get_depth(symbol, limit).await,
            Exchange::Toobit(e) => e.get_depth(symbol, limit).await,
            Exchange::Custom(e) => e.get_depth(symbol, limit).await,
            Exchange::Simu(e) => e.get_depth(symbol, limit).await,
            Exchange::None => todo!(),
        }
    }

    pub async fn get_order(&mut self, id: OrderId) -> Result<Order, ExchangeError> {
        match self {
            Exchange::Xt(e) => e.get_order(id).await,
            Exchange::Mexc(e) => e.get_order(id).await,
            Exchange::Lighter(e) => e.get_order(id).await,
            Exchange::Lbank(e) => e.get_order(id).await,
            Exchange::Kcex(e) => e.get_order(id).await,
            Exchange::Okx(e) => e.get_order(id).await,
            Exchange::Paradex(e) => e.get_order(id).await,
            Exchange::Weex(e) => e.get_order(id).await,
            Exchange::Gate(e) => e.get_order(id).await,
            Exchange::Grvt(e) => e.get_order(id).await,
            Exchange::Htx(e) => e.get_order(id).await,
            Exchange::Hyperliquid(e) => e.get_order(id).await,
            Exchange::Bybit(e) => e.get_order(id).await,
            Exchange::Bitget(e) => e.get_order(id).await,
            Exchange::Bitmart(e) => e.get_order(id).await,
            Exchange::Bitmex(e) => e.get_order(id).await,
            Exchange::Bitunix(e) => e.get_order(id).await,
            Exchange::Binance(e) => e.get_order(id).await,
            Exchange::Aster(e) => e.get_order(id).await,
            Exchange::Aden(e) => e.get_order(id).await,
            Exchange::Dex(e) => e.get_order(id).await,
            Exchange::Dydx(e) => e.get_order(id).await,
            Exchange::Coinw(e) => e.get_order(id).await,
            Exchange::Toobit(e) => e.get_order(id).await,
            Exchange::Custom(e) => e.get_order(id).await,
            Exchange::Simu(e) => e.get_order(id).await,
            Exchange::None => todo!(),
        }
    }

    pub async fn place_order(&mut self, symbol: &Symbol, order_req: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        match self {
            Exchange::Xt(e) => e.place_order(symbol, order_req).await,
            Exchange::Mexc(e) => e.place_order(symbol, order_req).await,
            Exchange::Lighter(e) => e.place_order(symbol, order_req).await,
            Exchange::Lbank(e) => e.place_order(symbol, order_req).await,
            Exchange::Kcex(e) => e.place_order(symbol, order_req).await,
            Exchange::Okx(e) => e.place_order(symbol, order_req).await,
            Exchange::Paradex(e) => e.place_order(symbol, order_req).await,
            Exchange::Weex(e) => e.place_order(symbol, order_req).await,
            Exchange::Gate(e) => e.place_order(symbol, order_req).await,
            Exchange::Grvt(e) => e.place_order(symbol, order_req).await,
            Exchange::Htx(e) => e.place_order(symbol, order_req).await,
            Exchange::Hyperliquid(e) => e.place_order(symbol, order_req).await,
            Exchange::Bybit(e) => e.place_order(symbol, order_req).await,
            Exchange::Bitget(e) => e.place_order(symbol, order_req).await,
            Exchange::Bitmart(e) => e.place_order(symbol, order_req).await,
            Exchange::Bitmex(e) => e.place_order(symbol, order_req).await,
            Exchange::Bitunix(e) => e.place_order(symbol, order_req).await,
            Exchange::Binance(e) => e.place_order(symbol, order_req).await,
            Exchange::Aster(e) => e.place_order(symbol, order_req).await,
            Exchange::Aden(e) => e.place_order(symbol, order_req).await,
            Exchange::Dex(e) => e.place_order(symbol, order_req).await,
            Exchange::Dydx(e) => e.place_order(symbol, order_req).await,
            Exchange::Coinw(e) => e.place_order(symbol, order_req).await,
            Exchange::Toobit(e) => e.place_order(symbol, order_req).await,
            Exchange::Custom(e) => e.place_order(symbol, order_req).await,
            Exchange::Simu(e) => e.place_order(symbol, order_req).await,
            Exchange::None => todo!(),
        }
    }

    pub async fn cancel_order(&mut self, id: OrderId) -> Result<OrderId, ExchangeError> {
        match self {
            Exchange::Xt(e) => e.cancel_order(id).await,
            Exchange::Mexc(e) => e.cancel_order(id).await,
            Exchange::Lighter(e) => e.cancel_order(id).await,
            Exchange::Lbank(e) => e.cancel_order(id).await,
            Exchange::Kcex(e) => e.cancel_order(id).await,
            Exchange::Okx(e) => e.cancel_order(id).await,
            Exchange::Paradex(e) => e.cancel_order(id).await,
            Exchange::Weex(e) => e.cancel_order(id).await,
            Exchange::Gate(e) => e.cancel_order(id).await,
            Exchange::Grvt(e) => e.cancel_order(id).await,
            Exchange::Htx(e) => e.cancel_order(id).await,
            Exchange::Hyperliquid(e) => e.cancel_order(id).await,
            Exchange::Bybit(e) => e.cancel_order(id).await,
            Exchange::Bitget(e) => e.cancel_order(id).await,
            Exchange::Bitmart(e) => e.cancel_order(id).await,
            Exchange::Bitmex(e) => e.cancel_order(id).await,
            Exchange::Bitunix(e) => e.cancel_order(id).await,
            Exchange::Binance(e) => e.cancel_order(id).await,
            Exchange::Aster(e) => e.cancel_order(id).await,
            Exchange::Aden(e) => e.cancel_order(id).await,
            Exchange::Dex(e) => e.cancel_order(id).await,
            Exchange::Dydx(e) => e.cancel_order(id).await,
            Exchange::Coinw(e) => e.cancel_order(id).await,
            Exchange::Toobit(e) => e.cancel_order(id).await,
            Exchange::Custom(e) => e.cancel_order(id).await,
            Exchange::Simu(e) => e.cancel_order(id).await,
            Exchange::None => todo!(),
        }
    }
}

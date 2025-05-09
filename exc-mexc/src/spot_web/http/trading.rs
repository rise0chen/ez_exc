use exc_util::interface::{ApiKind, Method, Rest};
use exc_util::types::order::OrderSide;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeRequest {
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeResponse {
    ///交易对ID
    pub id: String,
    ///quotebase币ID
    pub mcd: String,
    ///base币ID
    pub cd: String,
    ///base币简称
    pub vn: String,
    ///quote币简称
    pub mn: String,
}

impl Rest for GetTradeRequest {
    type Response = GetTradeResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/platform/spot/market-v2/web/symbol/trade".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub currency_id: String,
    pub market_currency_id: String,
    pub trade_type: OrderSide,
    pub order_type: String,
    #[serde_as(as = "DisplayFromStr")]
    pub quantity: f64,
    //pub amount: Option<f64>,
    pub price: f64,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse(pub String);

impl Rest for PlaceOrderRequest {
    type Response = PlaceOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotWeb
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        if self.order_type == "MARKET_ORDER" {
            "/api/platform/spot/v4/order/place".to_string()
        } else {
            "/api/platform/spot/order/place".to_string()
        }
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    pub order_id: Option<String>,
    pub quantity: Option<f64>,
    //pub amount: Option<f64>,
    pub price: Option<f64>,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    pub new_order_id: String,
}

impl Rest for AmendOrderRequest {
    type Response = AmendOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotWeb
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/api/platform/spot/order/modify".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub order_id: Option<String>,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse;

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotWeb
    }
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        "/api/platform/spot/order/cancel/v2".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

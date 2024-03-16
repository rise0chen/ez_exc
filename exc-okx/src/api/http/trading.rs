use super::super::types::{FuturesOpenType, OrderSide, OrderStatus, OrderType};
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub inst_id: String,
    /// isolated：逐仓; cross：全仓; cash：非保证金
    pub td_mode: FuturesOpenType,
    /// buy：买， sell：卖
    pub side: OrderSide,
    /// market：市价单 limit：限价单 post_only：只做maker单 fok：全部成交或立即取消 ioc：立即成交并取消剩余 optimal_limit_ioc：市价委托立即成交并取消剩余（仅适用交割、永续）
    pub ord_type: OrderType,
    pub sz: f64,
    pub px: f64,
    pub cl_ord_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub ord_id: String,
}

impl Rest for PlaceOrderRequest {
    type Response = Vec<PlaceOrderResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/api/v5/trade/order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub inst_id: String,
    pub ord_id: Option<String>,
    pub cl_ord_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub inst_id: String,
    pub ord_id: String,
    pub cl_ord_id: Option<String>,
}

impl Rest for CancelOrderRequest {
    type Response = Vec<CancelOrderResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/api/v5/trade/cancel-order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    pub inst_id: String,
    pub ord_id: Option<String>,
    pub cl_ord_id: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    pub inst_id: String,
    pub ord_id: String,
    pub cl_ord_id: Option<String>,
    pub ord_type: OrderType,
    pub px: String,
    pub avg_px: String,
    #[serde_as(as = "DisplayFromStr")]
    pub sz: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub acc_fill_sz: f64,
    pub state: OrderStatus,
    pub td_mode: FuturesOpenType,
    pub side: OrderSide,
}

impl Rest for GetOrderRequest {
    type Response = Vec<GetOrderResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v5/trade/order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

use crate::futures_web::types::{OrderSide, PositionSide, TimeInForce};
use crate::response::{Data, List};
use exc_util::interface::{ApiKind, Method, Rest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaceOrderRequest {
    pub exchange_i_d: &'static str,
    pub instrument_i_d: String,
    pub local_i_d: Option<String>,
    pub direction: OrderSide,
    pub volume: Decimal,
    pub price: Decimal,
    /// 0正常 1只减仓
    pub offset_flag: PositionSide,
    /// 0限价
    pub order_price_type: i8,
    pub order_type: TimeInForce,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub order_sys_i_d: String,
}

impl Rest for PlaceOrderRequest {
    type Response = PlaceOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/cfd/cff/v1/SendOrderInsert".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CancelOrderRequest {
    /// 正常1
    pub action_flag: i8,
    pub order_sys_i_d: Option<String>,
    pub local_i_d: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub order_sys_i_d: String,
    pub local_i_d: Option<String>,
}

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/cfd/action/v1.0/SendOrderAction".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetOpenOrdersRequest {
    pub exchange_i_d: &'static str,
    pub product_group: &'static str,
    pub instrument_i_d: String,
    pub page_size: u16,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    #[serde(alias = "orderSysID")]
    pub order_sys_i_d: String,
    #[serde(alias = "localID")]
    pub local_i_d: Option<String>,
    #[serde(alias = "direction")]
    #[serde_as(as = "DisplayFromStr")]
    pub direction: i8,
    #[serde(alias = "volume")]
    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,
    #[serde(alias = "volumeTraded")]
    #[serde_as(as = "DisplayFromStr")]
    pub volume_traded: f64,
    #[serde(alias = "turnover")]
    #[serde_as(as = "DisplayFromStr")]
    pub turnover: f64,
    #[serde(alias = "fee")]
    #[serde_as(as = "DisplayFromStr")]
    pub fee: f64,
    #[serde(alias = "orderStatus")]
    #[serde_as(as = "DisplayFromStr")]
    pub order_status: i8,
}

impl Rest for GetOpenOrdersRequest {
    type Response = Data<Vec<Order>>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/cfd/query/v1.0/Order".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCloseOrdersRequest {
    pub instrument_i_d: String,
    pub page_size: u16,
}

impl Rest for GetCloseOrdersRequest {
    type Response = List<Order>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/cfd/order/v1/historyAllOrderPage".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

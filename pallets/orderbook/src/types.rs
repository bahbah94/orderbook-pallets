use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::RuntimeDebug;
use frame_system::*;
use scale_info::TypeInfo;

#[derive(
    Encode,
    Decode,
    Clone,
    Copy,
    RuntimeDebug,
    PartialEq,
    Eq,
    TypeInfo,
    MaxEncodedLen,
    DecodeWithMemTracking,
)]
pub enum OrderSide {
    Buy,
    Sell,
}
#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum OrderStatus {
    Filled,
    PartiallyFilled,
    Cancelled,
    Expired,
    Open,
}

#[derive(
    Encode,
    Decode,
    Clone,
    Copy,
    RuntimeDebug,
    PartialEq,
    Eq,
    TypeInfo,
    MaxEncodedLen,
    DecodeWithMemTracking,
)]
pub enum OrderType {
    Market,
    Limit,
    // will add the other stuff like IOK, Stop etc later
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]

pub struct MarketPair {
    pub base_asset: AssetId, // btc/usdt pair
    pub quote_asset: AssetId,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Order<T: Config> {
    pub order_id: OrderId,
    pub trader: T::AccountId,
    pub side: OrderSide,
    pub status: OrderStatus,
    pub order_type: OrderType,
    pub price: Amount,
    pub quantity: Amount,
    pub filled_quantity: Amount,
    pub ttl: Option<u32>,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Trade<T: Config> {
    pub trade_id: TradeId,
    pub buyer: T::AccountId,
    pub seller: T::AccountId,
    pub buy_order_id: OrderId,
    pub sell_order_id: OrderId,
    pub price: Amount,
    pub quantity: Amount,
}

pub type OrderId = u64;
pub type TradeId = u64;
pub type AssetId = u32;
pub type Amount = u128;

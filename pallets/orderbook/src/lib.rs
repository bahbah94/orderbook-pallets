
#![cfg_attr(not(feature = "std"), no_std)]

mod types;
pub use pallet::*;
use crate::types::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature="runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{Blake2_128Concat, pallet_prelude:: *};
	use frame_system::pallet_prelude::{OriginFor, *};
    use sp_core::Get;
    //use sp_runtime::legacy::byte_sized_error::DispatchError;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // maximum orders at any price level not sure if needed. this will be on the blockOrders cache
        #[pallet::constant]
        type MaxPendingOrders: Get<u32>;

        //keeping this to prevent DDoS attacks for cancelling too many orders
        #[pallet::constant]
        type MaxCancellationOrders: Get<u32>;

        #[pallet::constant]
        type MaxOrders: Get<u32>;

        #[pallet::constant]
        type MaxUserOrders: Get<u32>;

    }

     // ===========================
     // Persisten storage
     // ===========================
    #[pallet::storage]
    pub type Bids<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Amount,
        BoundedVec<OrderId, T::MaxOrders>,
        ValueQuery,
    >;
    
    #[pallet::storage]
    pub type Asks<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Amount,
        BoundedVec<OrderId, T::MaxOrders>,
        ValueQuery,
        >;

    // ===========================
     // Cache
     // ===========================

    #[pallet::storage]
    pub type PendingOrders<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Amount,
        BoundedVec<OrderId,T::MaxPendingOrders>,
        ValueQuery

    >;

    #[pallet::storage]
    pub type PendingBids<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Amount,
        BoundedVec<OrderId,T::MaxPendingOrders>,
        ValueQuery

    >;

    //Keeping this so that users can easily access their orders
    #[pallet::storage]
    pub type UserOrders<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<OrderId,T::MaxUserOrders>,
        ValueQuery
    >;

    #[pallet::storage]
    pub type NextOrderId<T: Config> = StorageValue<_, OrderId, ValueQuery>;

    #[pallet::storage]
    pub type NextTradeId<T: Config> = StorageValue<_, TradeId, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T:Config> {
        OrderPlaced{
            order_id: OrderId,
            side: OrderSide,
            price: Amount,
            quantity: Amount
        },
        TradeExecuted{
            trade_id: TradeId,
            buy_order_id: OrderId,
            sell_order_id: OrderId,
            buyer: T::AccountId,
            seller: T::AccountId,
            price: Amount,
            quantity: Amount,

        },
        OrderCancelled {
            order_id: OrderId,
            trader: T::AccountId,
        },
        OrderFilled {
            order_id: OrderId,
            trader: T::AccountId,
        },
        OrderPartiallyFilled {
            order_id: OrderId,
            trader: T::AccountId,
            filled_quantity: Amount,
            remaining_quantity: Amount,
        },
        // we are putting this event, so that we know its requested but it could not be processed perhaps
        CancellationRequested {
            order_id: OrderId,
            trader: T::AccountId,
        },
        MatchingCompleted {
            total_trades: u32,
            total_volume: Amount,
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Order not found
        OrderNotFound,
        
        /// Not the order owner
        NotOrderOwner,
        
        /// Order not active
        OrderNotActive,
        
        /// Price must be > 0
        InvalidPrice,
        
        /// Quantity must be > 0
        InvalidQuantity,
        
        /// Insufficient balance
        InsufficientBalance,
        
        /// Too many pending orders this block
        TooManyPendingOrders,
        
        /// Too many pending cancellations this block
        TooManyPendingCancellations,
        
        /// Arithmetic overflow
        ArithmeticOverflow,
        
        /// Arithmetic underflow
        ArithmeticUnderflow,
        
        /// Failed to unreserve funds
        FailedToUnreserveFunds,
        
        /// No matching orders
        NoMatchingOrders,
    }


    // ========================================
    // HOOKS FOR MATCHING
    // ========================================

    #[pallet::hooks]
    impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_n: BlockNumberFor<T>) {
            todo!("Actual matching")
            // Then we need to match stuff from block order

            // if not found look at the OBStorage

            // Clear the cache
        }
    }


    // ============================================================
    // EXTRINSICS
    // ============================================================

    #[pallet::call]
    impl<T:Config> Pallet<T> {
        /// Place a limit order
        #[pallet::call_index(0)]
        #[pallet::weight(10000)]
        pub fn place_order(
            origin: OriginFor<T>,
            side: OrderSide,
            price: Amount,
            quantity: Amount,
            ordertype: OrderType
            ) -> DispatchResult {
                //let trader = ensure_signed(origin)?;
                todo!("We need to process this to place orders")
            }

        #[pallet::call_index(1)]
        #[pallet::weight(10000)]
        pub fn cancel_order(
            origin: OriginFor<T>,
            orderid: OrderId,
        ) -> DispatchResult {
            //let trader = ensure_signed(origin)?;

            todo!("We need to process this to cancel orders ")
        }
    }
}
//! [`Config`]: ./trait.Config.html

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, ReservableCurrency};
pub use pallet::*;
use sp_std::prelude::*;
mod emotes;
mod tests;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{decl_error, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The currency trait.
        type Currency: ReservableCurrency<Self::AccountId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]

    pub enum Event<T: Config> {
        //RMRK_Example_Event(T::AccountId),
    }

    /// Error for the RMRK pallet.
    #[pallet::error]
    pub enum Error<T> {
        //EventName
        InvalidInput,
    }

    /// The lookup table for names.
    #[pallet::storage]
    pub(super) type NameOf<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, (Vec<u8>, BalanceOf<T>)>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(50_000_000)]
        pub fn hello_rmrk(origin: OriginFor<T>) -> DispatchResult {
            Err(Error::<T>::InvalidInput)?
        }
    }
}

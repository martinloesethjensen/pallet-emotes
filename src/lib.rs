//! [`Config`]: ./trait.Config.html

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, OnUnbalanced, ReservableCurrency};
pub use pallet::*;
use sp_runtime::traits::{StaticLookup, Zero};
use sp_std::prelude::*;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        pallet_prelude::*,
        traits::{EnsureOrigin, Get},
    };
    use frame_system::{ pallet_prelude::*};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The currency trait.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Reservation fee.
        #[pallet::constant]
        type ReservationFee: Get<BalanceOf<Self>>;

        /// What to do with slashed funds.
        type Slashed: OnUnbalanced<NegativeImbalanceOf<Self>>;

        /// The origin which may forcibly set or remove a name. Root can always do this.
        type ForceOrigin: EnsureOrigin<Self::Origin>;

        /// The minimum length a name may be.
        #[pallet::constant]
        type MinLength: Get<u32>;

        /// The maximum length a name may be.
        #[pallet::constant]
        type MaxLength: Get<u32>;
    }

    #[pallet::event]
    pub enum Event<T: Config> {
        //RMRK_Example_Event(T::AccountId),
    }

    /// Error for the RMRK pallet.
    #[pallet::error]
    pub enum Error<T> {
        //EventName
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
        pub fn test(origin: OriginFor<T>) -> DispatchResult {
            Ok(())
        }
    }
}

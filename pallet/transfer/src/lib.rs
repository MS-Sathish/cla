#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::ExistenceRequirement,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Zero;
    use sp_std::vec::Vec;
    use pallet_token::Pallet as AssetsPallet; // Import the token pallet

    type BalanceOf<T> = <T as pallet_token::Config>::Balance;
    type AssetIdOf<T> = <T as pallet_token::Config>::AssetId;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_token::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn locked_funds)]
    pub type LockedFunds<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, (AssetIdOf<T>, BalanceOf<T>), OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MintedAndLocked(T::AccountId, AssetIdOf<T>, BalanceOf<T>),
        Claimed(T::AccountId, AssetIdOf<T>),
        Locked(T::AccountId, AssetIdOf<T>, BalanceOf<T>),
        Burned(T::AccountId, AssetIdOf<T>)
    }

    #[pallet::error]
    pub enum Error<T> {
        NoLockedFunds,
        MintFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn mint_and_lock(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
            account: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin.clone())?; 

            let origin_clone = origin.clone();
    
            AssetsPallet::<T>::mint(origin, asset_id, account.clone(), amount)?;           
    
            AssetsPallet::<T>::lock(origin_clone, asset_id, account.clone(), amount)?;
    
            Self::deposit_event(Event::MintedAndLocked(account, asset_id, amount));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn claim_locked_funds(origin: OriginFor<T>,asset_id: AssetIdOf<T>) -> DispatchResult {
            let origin_clone = origin.clone();  

            let user = ensure_signed(origin)?;  

            AssetsPallet::<T>::unlock(origin_clone, asset_id)?;            

            Self::deposit_event(Event::Claimed(user, asset_id));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn lock_tokens(origin: OriginFor<T>, asset_id: AssetIdOf<T>,account: T::AccountId,amount: BalanceOf<T>) -> DispatchResult {
            let sender = ensure_signed(origin.clone())?; 
            AssetsPallet::<T>::lock(origin, asset_id, account.clone(), amount)?;
            Self::deposit_event(Event::Locked(account, asset_id, amount));
            Ok(())
        }
        
        #[pallet::weight(10_000)]
        pub fn burn(origin: OriginFor<T>, asset_id: AssetIdOf<T>,account: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin.clone())?; 
            AssetsPallet::<T>::burn_tokens(origin, asset_id, account.clone())?;          
            Self::deposit_event(Event::Burned(account, asset_id));
            Ok(())
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, traits::Currency};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

type BalanceOf<T> =
  <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
  use super::*;

  #[pallet::config]
  pub trait Config: frame_system::Config {
    type Currency: Currency<Self::AccountId>;
    #[pallet::constant]
    type TreasuryAccount: Get<Self::AccountId>;
  }

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::storage]
  #[pallet::getter(fn token_count)]
  pub(super) type TokenCount<T> = StorageValue<_, u64, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    TokenCreated(T::AccountId, u64, u64), // who, tier, supply
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn create_token(origin: OriginFor<T>, tier: u8, supply: u64) -> DispatchResult {
      let who = ensure_signed(origin)?;
      ensure!(tier >= 1 && tier <= 5, "Invalid tier");

      // Fee logic
      let flat_fee: BalanceOf<T> = match tier {
        1 => 60u32.into(),
        2 => 250u32.into(),
        3 => 500u32.into(),
        4 => 1000u32.into(),
        5 => 1500u32.into(),
        _ => 0u32.into(),
      };

      let percent_cut = match tier {
        1 => 2,
        2 => 2,
        3 => 2,
        4 => 3,
        5 => 3,
        _ => 0,
      };

      let cut_amount = supply * percent_cut as u64 / 100;

      T::Currency::transfer(&who, &T::TreasuryAccount::get(), flat_fee, frame_support::traits::ExistenceRequirement::KeepAlive)?;

      let id = TokenCount::<T>::get();
      TokenCount::<T>::put(id + 1);

      // Emit event
      Self::deposit_event(Event::TokenCreated(who, tier as u64, supply - cut_amount));
      Ok(())
    }
  }
}

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, traits::{Currency, ExistenceRequirement}};
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
    type BurnPercent: Get<u8>;
    #[pallet::constant]
    type DiversifyPercent: Get<u8>;
    #[pallet::constant]
    type DAOAccount: Get<Self::AccountId>;
  }

  #[pallet::pallet]
  pub struct Pallet<T>(_);

  #[pallet::storage]
  #[pallet::getter(fn total_treasury)]
  pub(super) type TreasuryTotal<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    TreasuryReceived(BalanceOf<T>),
    Burned(BalanceOf<T>),
    Diversified(BalanceOf<T>),
    SentToDAO(BalanceOf<T>),
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn receive(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
      let _ = ensure_signed(origin)?;

      let burn = amount * T::BurnPercent::get().into() / 100u32.into();
      let diversify = amount * T::DiversifyPercent::get().into() / 100u32.into();
      let dao_share = amount - burn - diversify;

      TreasuryTotal::<T>::mutate(|t| *t += amount);

      Self::deposit_event(Event::TreasuryReceived(amount));
      Self::deposit_event(Event::Burned(burn));
      Self::deposit_event(Event::Diversified(diversify));
      Self::deposit_event(Event::SentToDAO(dao_share));

      // Simulate treasury operations
      T::Currency::withdraw(&T::DAOAccount::get(), burn, frame_support::traits::WithdrawReasons::TRANSFER, ExistenceRequirement::AllowDeath)?;
      T::Currency::withdraw(&T::DAOAccount::get(), diversify, frame_support::traits::WithdrawReasons::TRANSFER, ExistenceRequirement::AllowDeath)?;
      T::Currency::deposit_creating(&T::DAOAccount::get(), dao_share);

      Ok(())
    }
  }
}

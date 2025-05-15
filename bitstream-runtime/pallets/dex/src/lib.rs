#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, traits::Currency};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

type BalanceOf<T> =
  <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
  use super::*;

  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
  pub struct TokenPair {
    pub token_a: Vec<u8>,
    pub token_b: Vec<u8>,
    pub rate: u64, // token_a per token_b
  }

  #[pallet::config]
  pub trait Config: frame_system::Config {
    type Currency: Currency<Self::AccountId>;
  }

  #[pallet::pallet]
  pub struct Pallet<T>(_);

  #[pallet::storage]
  #[pallet::getter(fn pairs)]
  pub(super) type Pairs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, TokenPair, OptionQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    SwapExecuted(Vec<u8>, Vec<u8>, BalanceOf<T>),
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn register_pair(origin: OriginFor<T>, a: Vec<u8>, b: Vec<u8>, rate: u64) -> DispatchResult {
      let _ = ensure_root(origin)?;
      let id = [a.clone(), b.clone()].concat();
      let pair = TokenPair { token_a: a, token_b: b, rate };
      Pairs::<T>::insert(id.clone(), pair);
      Ok(())
    }

    #[pallet::weight(10_000)]
    pub fn swap(origin: OriginFor<T>, a: Vec<u8>, b: Vec<u8>, amount: BalanceOf<T>) -> DispatchResult {
      let who = ensure_signed(origin)?;
      let id = [a.clone(), b.clone()].concat();

      let pair = Pairs::<T>::get(&id).ok_or(Error::<T>::PairNotFound)?;
      let result = amount * pair.rate.into();

      T::Currency::withdraw(&who, amount, frame_support::traits::WithdrawReasons::TRANSFER, ExistenceRequirement::KeepAlive)?;
      T::Currency::deposit_creating(&who, result);

      Self::deposit_event(Event::SwapExecuted(a, b, result));
      Ok(())
    }
  }

  #[pallet::error]
  pub enum Error<T> {
    PairNotFound,
  }
}

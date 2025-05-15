#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
  use super::*;

  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
  pub struct MediaItem {
    pub cid: Vec<u8>,
    pub uploader: Vec<u8>,
    pub timestamp: u64,
  }

  #[pallet::config]
  pub trait Config: frame_system::Config {}

  #[pallet::pallet]
  pub struct Pallet<T>(_);

  #[pallet::storage]
  #[pallet::getter(fn media_by_account)]
  pub(super) type Media<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<MediaItem>, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    ContentUploaded(T::AccountId, Vec<u8>),
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn upload(origin: OriginFor<T>, cid: Vec<u8>) -> DispatchResult {
      let who = ensure_signed(origin)?;
      let now = <frame_system::Pallet<T>>::block_number().saturated_into::<u64>();
      let mut items = Media::<T>::get(&who);

      items.push(MediaItem {
        cid: cid.clone(),
        uploader: who.encode(),
        timestamp: now,
      });

      Media::<T>::insert(&who, items);
      Self::deposit_event(Event::ContentUploaded(who, cid));
      Ok(())
    }
  }
}

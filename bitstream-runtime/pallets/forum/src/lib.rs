#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, traits::Get};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
  use super::*;

  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
  pub struct Post {
    pub author: Vec<u8>,
    pub cid: Vec<u8>, // IPFS hash
    pub timestamp: u64,
  }

  #[pallet::config]
  pub trait Config: frame_system::Config {}

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::storage]
  #[pallet::getter(fn forum_posts)]
  pub(super) type ForumPosts<T: Config> =
    StorageMap<_, Blake2_128Concat, u64, Vec<Post>, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    PostAdded(u64, Vec<u8>), // token_id, cid
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn add_post(origin: OriginFor<T>, token_id: u64, cid: Vec<u8>) -> DispatchResult {
      let who = ensure_signed(origin)?;
      let now = <frame_system::Pallet<T>>::block_number().saturated_into::<u64>();

      let mut posts = ForumPosts::<T>::get(token_id);
      posts.push(Post {
        author: who.encode(),
        cid,
        timestamp: now,
      });

      ForumPosts::<T>::insert(token_id, posts.clone());
      Self::deposit_event(Event::PostAdded(token_id, posts.last().unwrap().cid.clone()));
      Ok(())
    }
  }
}

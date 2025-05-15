#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, traits::Get};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
  use super::*;

  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
  pub struct Proposal {
    pub description: Vec<u8>,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub executed: bool,
  }

  #[pallet::config]
  pub trait Config: frame_system::Config {}

  #[pallet::pallet]
  pub struct Pallet<T>(_);

  #[pallet::storage]
  #[pallet::getter(fn proposals)]
  pub(super) type Proposals<T: Config> = StorageMap<_, Blake2_128Concat, u64, Proposal, OptionQuery>;

  #[pallet::storage]
  #[pallet::getter(fn proposal_count)]
  pub(super) type ProposalCount<T> = StorageValue<_, u64, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    ProposalCreated(u64),
    VoteCast(u64, bool),
    ProposalExecuted(u64),
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn create_proposal(origin: OriginFor<T>, description: Vec<u8>) -> DispatchResult {
      let _ = ensure_signed(origin)?;
      let id = ProposalCount::<T>::get();

      let new_proposal = Proposal {
        description,
        yes_votes: 0,
        no_votes: 0,
        executed: false,
      };

      Proposals::<T>::insert(id, new_proposal);
      ProposalCount::<T>::put(id + 1);
      Self::deposit_event(Event::ProposalCreated(id));
      Ok(())
    }

    #[pallet::weight(10_000)]
    pub fn vote(origin: OriginFor<T>, proposal_id: u64, approve: bool) -> DispatchResult {
      let _ = ensure_signed(origin)?;
      Proposals::<T>::try_mutate(proposal_id, |maybe_prop| {
        let prop = maybe_prop.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
        if approve {
          prop.yes_votes += 1;
        } else {
          prop.no_votes += 1;
        }
        Self::deposit_event(Event::VoteCast(proposal_id, approve));
        Ok(())
      })
    }

    #[pallet::weight(10_000)]
    pub fn execute_proposal(origin: OriginFor<T>, proposal_id: u64) -> DispatchResult {
      let _ = ensure_signed(origin)?;
      Proposals::<T>::try_mutate(proposal_id, |maybe_prop| {
        let prop = maybe_prop.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
        ensure!(!prop.executed, "Already executed");
        ensure!(prop.yes_votes > prop.no_votes, "Not approved");

        prop.executed = true;
        Self::deposit_event(Event::ProposalExecuted(proposal_id));
        Ok(())
      })
    }
  }

  #[pallet::error]
  pub enum Error<T> {
    ProposalNotFound,
  }
}

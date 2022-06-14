#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::OriginFor;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{AtLeast32Bit, MaybeHash};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type ClubId: Parameter
			+ Member
			+ Default
			+ Copy
			+ MaxEncodedLen
			+ AtLeast32Bit
			+ MaybeSerializeDeserialize
			+ MaybeHash;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn clubs)]
	// We could use `BTreeSet` here for storing members, but it would be less efficient.
	pub type Clubs<T: Config> =
		StorageDoubleMap<_, Blake2_128, T::ClubId, Blake2_128Concat, T::AccountId, ()>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub clubs: std::collections::HashMap<T::ClubId, Vec<T::AccountId>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { clubs: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			self.clubs.iter().for_each(|(c, xs)| {
				xs.into_iter().for_each(|x| <Clubs<T>>::insert(c, &x, ()));
			});
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new member added to the club. [club_id, member]
		MemberAdded(T::ClubId, T::AccountId),
		/// A member was removed from the club. [club_id, member]
		MemberRemoved(T::ClubId, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Member already exists.
		DuplicateMember,
		/// Member does not exist.
		MemberDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a new member (account) to the given club. Can only be called by `root`.
		/// Throws a `DuplicateMember` error, if the member was already in the club.
		/// Emits an event on success.
		#[pallet::weight(324 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn add_member(
			origin: OriginFor<T>,
			club_id: T::ClubId,
			member: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(!<Clubs<T>>::contains_key(&club_id, &member), Error::<T>::DuplicateMember);
			<Clubs<T>>::insert(club_id, &member, ());
			Self::deposit_event(Event::MemberAdded(club_id, member));
			Ok(())
		}

		/// Removes the given member (account) from the club if existed, otherwise throws an error.
		/// Can only be called by `root`. Emits an event on success.
		#[pallet::weight(174 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn remove_member(
			origin: OriginFor<T>,
			club_id: T::ClubId,
			member: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(<Clubs<T>>::contains_key(&club_id, &member), Error::<T>::MemberDoesNotExist);
			<Clubs<T>>::remove(club_id, &member);
			Self::deposit_event(Event::MemberRemoved(club_id, member));
			Ok(())
		}
	}
}

//! Benchmarking setup for pallet-clubs

use super::*;

#[allow(unused)]
use crate::Pallet as ClubsModule;
use frame_benchmarking::{benchmarks, account};
use frame_system::RawOrigin;

benchmarks! {
	add_member {
		let m in 0u32 .. 100;
		let c in 0u32 .. 2;
		let club_id = T::ClubId::from(c);
		let member: T::AccountId = account("testacc", 0, m);
	}: _(RawOrigin::Root, club_id, member.clone())
	verify {
		assert!(Clubs::<T>::get(club_id, member).is_some());
	}

	remove_member {
		let m in 0u32 .. 100;
		let c in 0u32 .. 2;
		let club_id = T::ClubId::from(c);
		let member: T::AccountId = account("testacc", 0, m);
		ClubsModule::<T>::add_member(RawOrigin::Root.into(), club_id, member.clone()).unwrap();
	}: _(RawOrigin::Root, club_id, member.clone())
	verify {
		assert!(Clubs::<T>::get(club_id, member).is_none());
	}

	impl_benchmark_test_suite!(Clubs, crate::mock::new_test_ext(), crate::mock::Test);
}

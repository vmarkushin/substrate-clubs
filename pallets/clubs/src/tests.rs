use crate::{mock::*, pallet, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::DispatchError;

#[test]
fn test_adding_members_to_club() {
	new_test_ext().execute_with(|| {
		let club_id = 1;
		let account_id = 1;
		assert_ok!(ClubsModule::add_member(Origin::root(), club_id, account_id));
		assert!(pallet::Clubs::<Test>::get(club_id, account_id).is_some());
		assert!(pallet::Clubs::<Test>::get(club_id + 1, account_id).is_none());
		assert_noop!(
			ClubsModule::add_member(Origin::root(), club_id, account_id),
			Error::<Test>::DuplicateMember
		);
		assert_noop!(
			ClubsModule::add_member(Origin::signed(1), club_id, account_id),
			DispatchError::BadOrigin
		);
	});
}

#[test]
fn test_removing_members_from_club() {
	new_test_ext().execute_with(|| {
		let club_id = 1;
		let account_id = 1;
		assert_ok!(ClubsModule::add_member(Origin::root(), club_id, account_id));
		assert_ok!(ClubsModule::remove_member(Origin::root(), club_id, account_id));
		assert_noop!(
			ClubsModule::remove_member(Origin::root(), club_id, account_id),
			Error::<Test>::MemberDoesNotExist
		);
		assert_noop!(
			ClubsModule::remove_member(Origin::signed(1), club_id, account_id),
			DispatchError::BadOrigin
		);
	});
}

// use super::*;
use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(1)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyID::<Test>::set(crate::KittyID::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InValidKittyId
		);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id_1 = 0;
		let kitty_id_2 = 1;
		let account_id = 1;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id_2 + 1);
		assert_eq!(KittiesModule::kitties(kitty_id_1).is_some(), true);
		assert_eq!(KittiesModule::kitties(kitty_id_2).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id_1), Some(account_id));
		assert_eq!(KittiesModule::kitty_owner(kitty_id_2), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id_1), None);
		assert_eq!(KittiesModule::kitty_parents(kitty_id_2), None);

		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id_1, kitty_id_2));

		let kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), Some((kitty_id_1, kitty_id_2)));

		crate::NextKittyID::<Test>::set(crate::KittyID::max_value());
		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id_1, kitty_id_2),
			Error::<Test>::InValidKittyId
		);
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id_1 = 1;
		let account_id_2 = 2;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id_1)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id_1));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id_1), account_id_2, kitty_id));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id_2));

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(account_id_1), account_id_2, kitty_id),
			Error::<Test>::NotOwner
		);
	});
}
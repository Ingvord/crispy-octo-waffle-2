use crate::{mock::*, Error, Event, Something, UserInteractions};
use frame_support::{assert_noop, assert_ok};

// Test successful counter increment
#[test]
fn it_works_for_increment() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Initialize the counter value to 0
		assert_ok!(TemplateModule::set_counter_value(RuntimeOrigin::root(), 0));

		// Increment the counter by 5
		assert_ok!(TemplateModule::increment(RuntimeOrigin::signed(1), 5));
		// Check that the event emitted matches the increment operation
		System::assert_last_event(
			Event::CounterIncremented {
				counter_value: 5,
				who: 1,
				incremented_amount: 5,
			}
				.into(),
		);
	});
}

// Verify increment is blocked when it would exceed max value
#[test]
fn increment_fails_for_max_value_exceeded() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Set counter value close to max (10)
		assert_ok!(TemplateModule::set_counter_value(RuntimeOrigin::root(), 7));
		// Ensure that incrementing by 4 exceeds max value (10) and fails
		assert_noop!(
            TemplateModule::increment(RuntimeOrigin::signed(1), 4),
            Error::<Test>::CounterValueExceedsMax // Expecting CounterValueExceedsMax error
        );
	});
}

// Ensure non-root accounts cannot set counter value
#[test]
fn set_counter_value_fails_for_non_root() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Ensure only root (privileged account) can set counter value
		assert_noop!(
            TemplateModule::set_counter_value(RuntimeOrigin::signed(1), 5), // non-root account
            sp_runtime::traits::BadOrigin // Expecting a BadOrigin error
        );
	});
}

// Ensure increment fails on u32 overflow
#[test]
fn increment_handles_overflow() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Set to max value
		assert_ok!(TemplateModule::set_counter_value(RuntimeOrigin::root(), 1));
		assert_noop!(
            TemplateModule::increment(RuntimeOrigin::signed(1), u32::MAX),
            Error::<Test>::CounterOverflow
        );
	});
}

// Check that user interactions are correctly tracked
#[test]
fn user_interactions_increment() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Initialize counter value to 0
		assert_ok!(TemplateModule::set_counter_value(RuntimeOrigin::root(), 0));

		// Increment by 5 and decrement by 2
		assert_ok!(TemplateModule::increment(RuntimeOrigin::signed(1), 5));
		assert_ok!(TemplateModule::decrement(RuntimeOrigin::signed(1), 2));

		// Check if the user interactions are correctly tracked
		assert_eq!(UserInteractions::<Test>::get(1).unwrap_or(0), 2); // User should have 2 interactions
	});
}

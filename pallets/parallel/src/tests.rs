use crate::{mock::*, EnlistedParticipant};
use frame_support::{assert_noop, assert_ok};
use sp_core::Pair;

#[test]
fn it_can_enlist() {
	sp_io::TestExternalities::default().execute_with(|| {
		let (pair1, _) = sp_core::sr25519::Pair::generate();
		let (pair2, _) = sp_core::sr25519::Pair::generate();

		let event_name = b"test";

		ParallelTasks::run_event(Origin::signed(1), event_name.to_vec()).expect("Failed to enlist");

		let participants = vec![
			EnlistedParticipant {
				account: pair1.public().to_vec(),
				signature: AsRef::<[u8]>::as_ref(&pair1.sign(event_name)).to_vec(),
			},
			EnlistedParticipant {
				account: pair2.public().to_vec(),
				signature: AsRef::<[u8]>::as_ref(&pair2.sign(event_name)).to_vec(),
			},
		];

		ParallelTasks::enlist_participants(Origin::signed(1), participants)
			.expect("Failed to enlist");

		assert_eq!(ParallelTasks::participants().len(), 2);
	});
}

#[test]
fn one_wrong_will_not_enlist_anyone() {
	sp_io::TestExternalities::default().execute_with(|| {
		let (pair1, _) = sp_core::sr25519::Pair::generate();
		let (pair2, _) = sp_core::sr25519::Pair::generate();
		let (pair3, _) = sp_core::sr25519::Pair::generate();

		let event_name = b"test";

		ParallelTasks::run_event(Origin::signed(1), event_name.to_vec()).expect("Failed to enlist");

		let participants = vec![
			EnlistedParticipant {
				account: pair1.public().to_vec(),
				signature: AsRef::<[u8]>::as_ref(&pair1.sign(event_name)).to_vec(),
			},
			EnlistedParticipant {
				account: pair2.public().to_vec(),
				signature: AsRef::<[u8]>::as_ref(&pair2.sign(event_name)).to_vec(),
			},
			// signing wrong event
			EnlistedParticipant {
				account: pair3.public().to_vec(),
				signature: AsRef::<[u8]>::as_ref(&pair3.sign(&[])).to_vec(),
			},
		];

		ParallelTasks::enlist_participants(Origin::signed(1), participants)
			.expect("Failed to enlist");

		assert_eq!(ParallelTasks::participants().len(), 0);
	});
}

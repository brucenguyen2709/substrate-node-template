// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Parallel Tasks Example Pallet
//!
//! This example pallet demonstrates parallelizing validation of the enlisted participants
//! (see `enlist_participants` dispatch).
//!
//! **This pallet serves as an example and is not meant to be used in production.**

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Decode, Encode};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::{ensure_signed, pallet_prelude::*};

	use sp_core::Bytes;
	use sp_core::Public;
	use sp_runtime::traits::Verify;
	use sp_runtime::RuntimeDebug;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The overarching dispatch call type.
		type Call: From<Call<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	//#[pallet::metadata(AccountId<T> = "AccountId")]
	pub enum Event<T: Config> {
		/// Event emitted when a proof has been claimed. [who, claim]
		ClaimCreated(<T as frame_system::Config>::AccountId, Vec<u8>),
		/// Event emitted when a claim is revoked by the owner. [who, claim]
		ClaimRevoked(<T as frame_system::Config>::AccountId, Vec<u8>),
	}

	/// A vector of current participants
	///
	/// To enlist someone to participate, signed payload should be sent to `enlist`.
	#[pallet::storage]
	#[pallet::getter(fn participants)]
	pub(super) type Participants<T: Config> = StorageValue<_, Vec<Vec<u8>>, ValueQuery>;

	/// Current event id to enlist participants to.
	#[pallet::storage]
	#[pallet::getter(fn get_current_event_id)]
	pub(super) type CurrentEventId<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

	/// Request to enlist participant.
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct EnlistedParticipant {
		pub account: Vec<u8>,
		pub signature: Vec<u8>,
	}

	/// A public part of the pallet.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Get the new event running.
		#[pallet::weight(0)]
		pub fn run_event(origin: OriginFor<T>, id: Vec<u8>) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			<Participants<T>>::kill();
			<CurrentEventId<T>>::mutate(move |event_id| *event_id = id);
			Ok(().into())
		}

		/// Submit list of participants to the current event.
		///
		/// The example utilizes parallel execution by checking half of the
		/// signatures in spawned task.
		#[pallet::weight(0)]
		pub fn enlist_participants(
			origin: OriginFor<T>,
			participants: Vec<EnlistedParticipant>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;

			if Self::validate_participants_parallel(&<CurrentEventId<T>>::get(), &participants[..])
			{
				for participant in participants {
					<Participants<T>>::append(participant.account);
				}
			}
			Ok(().into())
		}
	}

	impl EnlistedParticipant {
		fn verify(&self, event_id: &[u8]) -> bool {
			match sp_core::sr25519::Signature::try_from(&self.signature[..]) {
				Ok(signature) => {
					match sp_core::sr25519::Public::from_slice(self.account.as_ref()) {
						Err(()) => false,
						Ok(signer) => signature.verify(event_id, &signer),
					}
				}
				_ => false,
			}
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn validate_participants_parallel(
			event_id: &[u8],
			participants: &[EnlistedParticipant],
		) -> bool {
			fn spawn_verify(data: Vec<u8>) -> Vec<u8> {
				let stream = &mut &data[..];
				let event_id = Vec::<u8>::decode(stream).expect("Failed to decode");
				let participants =
					Vec::<EnlistedParticipant>::decode(stream).expect("Failed to decode");

				for participant in participants {
					if !participant.verify(&event_id) {
						return false.encode();
					}
				}
				true.encode()
			}

			let mut async_payload = Vec::new();
			event_id.encode_to(&mut async_payload);
			participants[..participants.len() / 2].encode_to(&mut async_payload);

			let handle = sp_tasks::spawn(spawn_verify, async_payload);
			let mut result = true;

			for participant in &participants[participants.len() / 2 + 1..] {
				if !participant.verify(event_id) {
					result = false;
					break;
				}
			}

			bool::decode(&mut &handle.join()[..]).expect("Failed to decode result") && result
		}
	}
}

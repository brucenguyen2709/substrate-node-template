#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, log, pallet_prelude::*};
	use frame_support::log::log;
	use frame_system::pallet_prelude::*;
	use frame_support::sp_runtime::traits::Zero;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant] // put the constant in metadata
		/// Maximum amount added per invocation.
		type MaxAddend: Get<u32>;

		/// Frequency with which the stored value is deleted.
		type ClearFrequency: Get<Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn single_value)]
	pub(super) type SingleValue<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The value has ben added to. The parameters are
		/// (initial amount, amount added, final amount).
		Added(u32, u32, u32),
		/// The value has been cleared. The parameter is the value before clearing.
		Cleared(u32)
	}

	#[pallet::error]
	pub enum Error<T> {
		/// An operation would lead to an overflow.
		Overflow
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn on_finalize(n: T::BlockNumber) {
			if (n % T::ClearFrequency::get()).is_zero() {
				let current_value = <SingleValue<T>>::get();
				<SingleValue<T>>::put(0u32);
				Self::deposit_event(Event::Cleared(current_value));
			}
		}
	}

	// Extrinsics callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn add_value(
			origin: OriginFor<T>,
			val_to_add: u32
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;

			println!("MaxAddend: {:?}.", T::MaxAddend::get());
			log::info!("MaxAddend: {:?}.", T::MaxAddend::get());
			ensure!(val_to_add <= T::MaxAddend::get(), "value must be <= maximum add amount constant");

			// previous value got
			let c_val = SingleValue::<T>::get();

			// checks for overflow when new value added
			let result = c_val.checked_add(val_to_add).ok_or(Error::<T>::Overflow)?;

			<SingleValue<T>>::put(result);
			Self::deposit_event(Event::Added(c_val, val_to_add, result));
			Ok(().into())
		}
	}
}

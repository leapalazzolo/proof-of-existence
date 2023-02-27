#![cfg_attr(not(feature = "std"), no_std)]
// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a claim has been created.
		ClaimCreated { who: T::AccountId, claim: T::Hash },
		/// Event emitted when a claim is revoked by the owner.
		ClaimRevoked { who: T::AccountId, claim: T::Hash },
	}
	#[pallet::error]   // <-- Step 4. code block will replace this.
	#[pallet::storage] // <-- Step 5. code block will replace this.
	#[pallet::call]    // <-- Step 6. code block will replace this.
}

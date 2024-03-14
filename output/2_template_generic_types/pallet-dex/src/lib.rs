// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
use frame_support::traits::fungible;
use frame_support::traits::fungibles;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
#[cfg(test)]
mod tests;

// Define type aliases for easier access
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type AssetIdOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
	<T as frame_system::Config>::AccountId,
>>::AssetId;

pub type BalanceOf<T> = <<T as Config>::NativeBalance as fungible::Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub type AssetBalanceOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// Type to access the Balances Pallet
		type NativeBalance: fungible::Inspect<Self::AccountId>
		+ fungible::Mutate<Self::AccountId>
		+ fungible::hold::Inspect<Self::AccountId>
		+ fungible::hold::Mutate<Self::AccountId>
		+ fungible::freeze::Inspect<Self::AccountId>
		+ fungible::freeze::Mutate<Self::AccountId>;

		// TODO: Define the Fungibles type here
	}

	/// A storage item for this pallet.
	#[pallet::storage]
	pub type SomeItem<T> = StorageValue<_, u32>;

	/// A storage map for this pallet.
	#[pallet::storage]
	pub type SomeMap<T> = StorageMap<_, Blake2_128Concat, u32, u32>;

	/// Events that functions in this pallet can emit.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/* Pallet Event Variants Go Here */
	}

	/// Errors that can be returned by this pallet.
	#[pallet::error]
	pub enum Error<T> {
		/* Pallet Error Variants Go Here */
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/* User Callable Functions Go Here */
	}

	/// The pallet's internal functions.
	impl<T: Config> Pallet<T> {
		/* Internally Callable Functions Go Here */
	}
}

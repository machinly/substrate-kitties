#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
//
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
pub mod weights;

pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use sp_io::hashing::blake2_128;
    use frame_support::traits::Randomness;

    pub type KittyID = u32;

    #[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
    pub struct Kitty(pub [u8; 16]);

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Type representing the weight of this pallet
        // type WeightInfo: WeightInfo;
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
    }

    // The pallet's runtime storage items.
    // https://docs.substrate.io/main-docs/build/runtime-storage/
    #[pallet::storage]
    #[pallet::getter(fn next_kitty_id)]
    // Learn more about declaring storage items:
    // https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
    pub type NextKittyID<T> = StorageValue<_, KittyID, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyID, Kitty>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_owner)]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyID, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_parents)]
    pub type KittyParent<T: Config> = StorageMap<_, Blake2_128Concat, KittyID, (KittyID, KittyID), OptionQuery>;


    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreated {
            who: T::AccountId,
            kitty_id: KittyID,
            kitty: Kitty,
        },

        KittyBred {
            who: T::AccountId,
            kitty_id: KittyID,
            kitty: Kitty,
        },

        KittyTransfered {
            who: T::AccountId,
            recipient: T::AccountId,
            kitty_id: KittyID,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Errors should have helpful documentation associated with them.
        InValidKittyId,
        SameKittyId,
        NotOwner,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/main-docs/build/origins/
            let who = ensure_signed(origin)?;

            let kitty_id = Self::get_next_id()?;
            let kitty = Kitty(Default::default());

            Kitties::<T>::insert(kitty_id, kitty.clone());
            KittyOwner::<T>::insert(kitty_id, who.clone());

            Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn breed(origin: OriginFor<T>, kitty_id_1: KittyID, kitty_id_2: KittyID) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);

            ensure!(KittyOwner::<T>::contains_key(kitty_id_1), Error::<T>::InValidKittyId);
            ensure!(KittyOwner::<T>::contains_key(kitty_id_2), Error::<T>::InValidKittyId);

            ensure!(KittyOwner::<T>::get(kitty_id_1) == Some(who.clone()), Error::<T>::NotOwner);
            ensure!(KittyOwner::<T>::get(kitty_id_2) == Some(who.clone()), Error::<T>::NotOwner);

            let kitty_id = Self::get_next_id()?;

            let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InValidKittyId)?;
            let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InValidKittyId)?;

            let selector = Self::random_value(&who);
            let mut data = [0u8; 16];
            for i in 0..kitty1.0.len() {
                data[i] = (selector[i] & kitty1.0[i]) | (!selector[i] & kitty2.0[i]);
            }

            let kitty = Kitty(data);

            Kitties::<T>::insert(kitty_id, kitty.clone());
            KittyOwner::<T>::insert(kitty_id, who.clone());
            KittyParent::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));

            Self::deposit_event(Event::KittyBred { who, kitty_id, kitty: kitty });

            Ok(())
        }


        #[pallet::call_index(2)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn transfer(origin: OriginFor<T>, recipient: T::AccountId, kitty_id: KittyID) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(KittyOwner::<T>::contains_key(kitty_id), Error::<T>::InValidKittyId);

            let owner = KittyOwner::<T>::get(kitty_id).ok_or(Error::<T>::InValidKittyId)?;
            ensure!(owner == who.clone(), Error::<T>::NotOwner);

            KittyOwner::<T>::insert(kitty_id, recipient.clone());
            Self::deposit_event(Event::KittyTransfered { who, recipient, kitty_id });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn get_next_id() -> Result<KittyID, DispatchError> {
            NextKittyID::<T>::try_mutate(|next_id| -> Result<KittyID, DispatchError> {
                let current_id = *next_id;
                *next_id = next_id.checked_add(1).ok_or::<DispatchError>(Error::<T>::InValidKittyId.into())?;
                Ok(current_id)
            })
        }

        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                sender.clone(),
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

// 存证相关
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber),
    >;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimTransfered(T::AccountId, Vec<u8>),
	}

    #[pallet::error]
	pub enum Error<T> {
		ProofAlreadyClaimed,
        NoSuchProof,
        NotProofOwner,
	}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
        ) -> DispatchResultWithPostInfo {

            let sender = ensure_signed(origin)?;
               
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyClaimed);

            let current_block = frame_system::Pallet::<T>::block_number();

            // Store the proof with the sender and block number.
            Proofs::<T>::insert(&claim, (sender.clone(), current_block));

            // Emit an event that the claim was created.
            Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            // Get owner of the claim.
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::NoSuchProof)?;

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // Remove claim from storage.
            Proofs::<T>::remove(&claim);

            // Emit an event that the claim was erased.
            Self::deposit_event(Event::ClaimRevoked(sender, claim));

            Ok(().into())
        }

        // 转移凭证

        // 转移存证
        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
            transfer: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // 校验sender
            let sender = ensure_signed(origin)?;

            // 判断存证存在
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::NoSuchProof)?;

            // 校验存证所属正确
            ensure!(owner==sender,Error::<T>::NotProofOwner);

            // 更改所属地址
            let current_block = frame_system::Pallet::<T>::block_number();
            Proofs::<T>::insert(
                &claim,
                (transfer.clone(), current_block));

            // 触发事件
            Self::deposit_event(Event::ClaimTransfered(sender, claim));

            Ok(().into())
        }
    }

}


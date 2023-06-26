use super::*;
use frame_support::{log, traits::OnRuntimeUpgrade};
use sp_runtime::Saturating;

pub mod v1 {
    use frame_support::{pallet_prelude::*, weights::Weight};

    use super::*;

    #[derive(Decode)]
    pub struct OldCommunity<AccountId, Hash, NameLimit: Get<u32>, DescriptionLimit: Get<u32>> {
        pub founder: AccountId,
        pub logo: Option<Vec<u8>>,
        pub name: BoundedVec<u8, NameLimit>,
        pub description: BoundedVec<u8, DescriptionLimit>,
        pub members: Vec<AccountId>,
        pub metadata: Option<CommunityMetaData<AccountId, Hash>>,
    }

    impl<AccountId, Hash, NameLimit: Get<u32>, DescriptionLimit: Get<u32>> OldCommunity<AccountId, Hash, NameLimit, DescriptionLimit> {
        fn migrate_to_v1(self, random_value: Hash) -> Community<AccountId, Hash, NameLimit, DescriptionLimit> {

            Community {
                founder: self.founder,
                logo: self.logo,
                name: self.name,
                description: self.description,
                members: self.members,
                metadata: self.metadata,
                reference_id: random_value
            }
        }
    }

    pub struct MigrateToV1<T>(sp_std::marker::PhantomData<T>);
    impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {

        fn on_runtime_upgrade() -> Weight {
            let current_version = Pallet::<T>::current_storage_version();
            let onchain_version = Pallet::<T>::on_chain_storage_version();

            if onchain_version == 0 && current_version == 1 {
                let mut translated = 0u64;
                let mut nonce = Nonce::<T>::get();
                Communities::<T>::translate::<
                    OldCommunity<T::AccountId, T::Hash, T::NameLimit, T::DescriptionLimit>,
                    _,
                >(|_key, old_value| {
                    translated.saturating_inc();
                    // Random value.
                    nonce.saturating_inc();
                    Nonce::<T>::put(nonce);
                    let nonce = nonce.encode();
                    let (random_value, _) = T::MyRandomness::random(&nonce);
                    Some(old_value.migrate_to_v1(random_value))
                });
                current_version.put::<Pallet<T>>();
                log::info!(
					target: LOG_TARGET,
					"Upgraded {} pools, storage to version {:?}",
					translated,
					current_version
				);
                T::DbWeight::get().reads_writes(translated + 1, translated + 1)
            } else {
                log::info!(
					target: LOG_TARGET,
					"Migration did not execute. This probably should be removed"
				);
                T::DbWeight::get().reads(1)
            }
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
            frame_support::ensure!(
				Pallet::<T>::on_chain_storage_version() == 0,
				"must upgrade linearly"
			);
            let prev_count = Communities::<T>::iter().count();
            Ok((prev_count as u32).encode())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade(prev_count: Vec<u8>) -> Result<(), &'static str> {
            let prev_count: u32 = Decode::decode(&mut prev_count.as_slice()).expect(
                "the state parameter should be something that was generated by pre_upgrade",
            );
            let post_count = Communities::<T>::iter().count() as u32;
            assert_eq!(
                prev_count, post_count,
                "the community count before and after the migration should be the same"
            );

            let current_version = Pallet::<T>::current_storage_version();
            let onchain_version = Pallet::<T>::on_chain_storage_version();

            frame_support::ensure!(current_version == 1, "must_upgrade");
            assert_eq!(
                current_version, onchain_version,
                "after migration, the current_version and onchain_version should be the same"
            );

            Ok(())
        }
    }
}


#[cfg(test)]
#[cfg(feature = "try-runtime")]
mod test {
    use frame_support::pallet_prelude::StorageVersion;
    use super::*;
    use crate::mock::{Test as T, *};

    #[test]
    fn migration_works() {
        new_test_ext().execute_with(|| {
            assert_eq!(StorageVersion::get::<Pallet<T>>(), 0);

            create_community();
            setup_blocks(5);
            create_community();

            assert_eq!(Communities::<T>::iter_values().count() as u32, 2);

            let state = v1::MigrateToV1::<T>::pre_upgrade().unwrap();
            let _w = v1::MigrateToV1::<T>::on_runtime_upgrade();
            v1::MigrateToV1::<T>::post_upgrade(state).unwrap();

            assert_eq!(Communities::<T>::iter_values().count() as u32, 2);
            assert_eq!(StorageVersion::get::<Pallet<T>>(), 1);
            assert_ne!(Some(Communities::<Test>::get(1).unwrap().reference_id), Some(Communities::<Test>::get(0).unwrap().reference_id));
        });
    }
}
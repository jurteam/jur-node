use super::*;
use frame_support::{log, traits::OnRuntimeUpgrade};
use sp_runtime::Saturating;

pub mod v8 {
	use frame_support::{pallet_prelude::*, weights::Weight};

	use super::*;
	#[derive(Decode)]
	pub struct OldCommunity<
		AccountId,
		NameLimit: Get<u32>,
		DescriptionLimit: Get<u32>,
		TagLimit: Get<u32>,
		ColorLimit: Get<u32>,
	> {
		pub founder: AccountId,
		pub logo: Option<Vec<u8>>,
		pub name: BoundedVec<u8, NameLimit>,
		pub description: BoundedVec<u8, DescriptionLimit>,
		pub members: Vec<AccountId>,
		pub metadata: Option<OldCommunityMetaData<AccountId>>,
		pub reference_id: [u8; 16],
		pub category: Category,
		pub tag: BoundedVec<u8, TagLimit>,
		pub primary_color: BoundedVec<u8, ColorLimit>,
		pub secondary_color: BoundedVec<u8, ColorLimit>,
	}

	#[derive(Decode)]
	pub struct OldCommunityMetaData<AccountId> {
		pub community_type: Option<CommunityType<AccountId>>,
		pub customs: Option<Vec<Vec<u8>>>,
		pub languages: Option<Vec<Vec<u8>>>,
		pub norms: Option<Vec<Vec<u8>>>,
		pub religions: Option<Vec<Vec<u8>>>,
		pub territories: Option<Vec<Vec<u8>>>,
		pub traditions: Option<Vec<Vec<u8>>>,
		pub values: Option<Vec<Vec<u8>>>,
	}

	pub struct MigrateToV8<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV8<T> {
		fn on_runtime_upgrade() -> Weight {
			let current_version = Pallet::<T>::current_storage_version();
			let onchain_version = Pallet::<T>::on_chain_storage_version();

			if onchain_version == 7 && current_version == 8 {
				let mut translated = 0u64;
				Communities::<T>::translate::<
					OldCommunity<
						T::AccountId,
						T::NameLimit,
						T::DescriptionLimit,
						T::TagLimit,
						T::ColorLimit,
					>,
					_,
				>(|_key, old_value| {
					translated.saturating_inc();

					let mut community_type: Option<CommunityType<T::AccountId>> = None;
					let mut meta_data: Option<CommunityMetaData<T::StringLimit>> = None;
					if let Some(meta) = old_value.metadata {
						community_type = meta.community_type;

						let mut customs: Option<Vec<Customs<T::StringLimit>>> = None;
						let mut languages: Option<Vec<Languages<T::StringLimit>>> = None;
						let mut norms: Option<Vec<Norms<T::StringLimit>>> = None;
						let mut religions: Option<Vec<Religions<T::StringLimit>>> = None;
						let mut territories: Option<Vec<Territories<T::StringLimit>>> = None;
						let mut traditions: Option<Vec<Traditions<T::StringLimit>>> = None;
						let mut values: Option<Vec<Values<T::StringLimit>>> = None;

						if let Some(cus) = meta.customs {
							customs = Some(
								cus.into_iter()
									.map(|c| {
										let new_custom =
											c.clone().try_into().unwrap_or_else(|_| {
												log::error!(
													target: LOG_TARGET,
													"Failed to convert custom"
												);
												Default::default()
											});
										Customs(new_custom)
									})
									.collect::<Vec<Customs<T::StringLimit>>>(),
							);
						}

						if let Some(old_value) = meta.languages {
							languages = Some(
								old_value
									.into_iter()
									.map(|c| {
										let new_value =
											c.try_into().unwrap_or_else(|_| {
												log::error!(
													target: LOG_TARGET,
													"Failed to convert language"
												);
												Default::default()
											});
										Languages(new_value)
									})
									.collect::<Vec<Languages<T::StringLimit>>>(),
							);
						}

						if let Some(old_value) = meta.norms {
							norms = Some(
								old_value
									.into_iter()
									.map(|c| {
										let new_value =
											c.try_into().unwrap_or_else(|_| {
												log::error!(
													target: LOG_TARGET,
													"Failed to convert norms"
												);
												Default::default()
											});
										Norms(new_value)
									})
									.collect::<Vec<Norms<T::StringLimit>>>(),
							);
						}

						if let Some(old_value) = meta.religions {
							religions = Some(
								old_value
									.into_iter()
									.map(|c| {
										let new_value =
											c.try_into().unwrap_or_else(|_| {
												log::error!(
													target: LOG_TARGET,
													"Failed to convert religions"
												);
												Default::default()
											});
										Religions(new_value)
									})
									.collect::<Vec<Religions<T::StringLimit>>>(),
							);
						}

						if let Some(old_value) = meta.territories {
							territories = Some(
								old_value
									.into_iter()
									.map(|c| {
										let new_value =
											c.try_into().unwrap_or_else(|_| {
												log::error!(
													target: LOG_TARGET,
													"Failed to convert territories"
												);
												Default::default()
											});
										Territories(new_value)
									})
									.collect::<Vec<Territories<T::StringLimit>>>(),
							);
						}

						if let Some(old_value) = meta.traditions {
							traditions = Some(
								old_value
									.into_iter()
									.map(|c| {
										let new_value =
											c.try_into().unwrap_or_else(|_| {
												log::error!(
													target: LOG_TARGET,
													"Failed to convert traditions"
												);
												Default::default()
											});
										Traditions(new_value)
									})
									.collect::<Vec<Traditions<T::StringLimit>>>(),
							);
						}

						if let Some(old_value) = meta.values {
							values = Some(
								old_value
									.into_iter()
									.map(|c| {
										let new_value =
											c.try_into().unwrap_or_else(|_| {
												log::error!(
													target: LOG_TARGET,
													"Failed to convert values"
												);
												Default::default()
											});
										Values(new_value)
									})
									.collect::<Vec<Values<T::StringLimit>>>(),
							);
						}

						meta_data = Some(CommunityMetaData {
							customs,
							languages,
							norms,
							religions,
							territories,
							traditions,
							values,
						});
					}

					let bounded_logo: BoundedVec<u8, T::LogoLimit> =
						if let Some(logo) = old_value.logo {
							logo.try_into().unwrap_or_else(|_| {
								log::error!(
													target: LOG_TARGET,
													"Failed to convert logo"
												);
								Default::default()
							})
						} else {
							Default::default()
						};

					Some(Community {
						founder: old_value.founder,
						logo: bounded_logo,
						name: old_value.name,
						description: old_value.description,
						members: old_value.members,
						metadata: meta_data,
						reference_id: old_value.reference_id,
						category: old_value.category,
						tag: old_value.tag,
						primary_color: old_value.primary_color,
						secondary_color: old_value.secondary_color,
						community_type,
					})
				});
				current_version.put::<Pallet<T>>();
				log::info!(
					target: LOG_TARGET,
					"Community Upgraded {} pools, storage to version {:?}",
					translated,
					current_version
				);
				T::DbWeight::get().reads_writes(translated + 1, translated + 1)
			} else {
				log::info!(
					target: LOG_TARGET,
					"Community Migration did not execute. This probably should be removed"
				);
				T::DbWeight::get().reads(1)
			}
		}

		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
			frame_support::ensure!(
				Pallet::<T>::on_chain_storage_version() == 1,
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

			frame_support::ensure!(current_version == 2, "must_upgrade");
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
	use super::*;
	use crate::mock::{Test as T, *};
	use frame_support::pallet_prelude::StorageVersion;

	#[test]
	fn migration_works() {
		new_test_ext().execute_with(|| {
			assert_eq!(StorageVersion::get::<Pallet<T>>(), 0);

			create_community();
			setup_blocks(5);
			create_community();

			assert_eq!(Communities::<T>::iter_values().count() as u32, 2);

			let state = v3::MigrateToV3::<T>::pre_upgrade().unwrap();
			let _w = v3::MigrateToV3::<T>::on_runtime_upgrade();
			v3::MigrateToV3::<T>::post_upgrade(state).unwrap();

			assert_eq!(Communities::<T>::iter_values().count() as u32, 2);
			assert_eq!(StorageVersion::get::<Pallet<T>>(), 2);
			assert_ne!(
				Some(Communities::<Test>::get(1).unwrap().reference_id),
				Some(Communities::<Test>::get(0).unwrap().reference_id)
			);
		});
	}
}

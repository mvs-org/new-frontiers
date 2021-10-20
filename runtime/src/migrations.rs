use frame_support::{pallet_prelude::Get, traits::OnRuntimeUpgrade, weights::Weight};
use pallet_author_mapping::{migrations::TwoXToBlake, Config as AuthorMappingConfig};
use pallet_migrations::Migration;
use sp_std::{marker::PhantomData, prelude::*};

/// this module acts as a registry where each migration is defined. Each migration should implement
/// the "Migration" trait declared in the pallet-migrations crate

pub struct AuthorMappingTwoXToBlake<T>(PhantomData<T>);
impl<T: AuthorMappingConfig> Migration for AuthorMappingTwoXToBlake<T> {
	fn friendly_name(&self) -> &str {
		"MM_Author_Mapping_TwoXToBlake"
	}

	fn migrate(&self, _available_weight: Weight) -> Weight {
		TwoXToBlake::<T>::on_runtime_upgrade()
	}

	/// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade(&self) -> Result<(), &'static str> {
		TwoXToBlake::<T>::pre_upgrade()
	}

	/// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
	#[cfg(feature = "try-runtime")]
	fn post_upgrade(&self) -> Result<(), &'static str> {
		TwoXToBlake::<T>::post_upgrade()
	}
}

pub struct CommonMigrations<Runtime>(PhantomData<Runtime>);
impl<Runtime> Get<Vec<Box<dyn Migration>>> for CommonMigrations<Runtime>
where
	Runtime: pallet_author_mapping::Config,
{
	fn get() -> Vec<Box<dyn Migration>> {
		let migration_author_mapping_twox_to_blake = AuthorMappingTwoXToBlake::<Runtime> {
			0: Default::default(),
		};

		// TODO: this is a lot of allocation to do upon every get() call. this *should* be avoided
		// except when pallet_migrations undergoes a runtime upgrade -- but TODO

		vec![
			// Box::new(migration_evm_storage_from_hyperspace)
		]
	}
}
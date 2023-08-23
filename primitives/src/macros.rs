#[macro_export]
macro_rules! impl_incrementable {
	($($type:ty),+) => {
		$(
			impl Incrementable for $type {
				fn increment(&self) -> Self {
					let mut val = self.clone();
					val.saturating_inc();
					val
				}

				fn initial_value() -> Self {
					1
				}

				fn jur_community_reserve_slots() -> Self {
					4851
				}
			}
		)+
	};
}
pub use impl_incrementable;

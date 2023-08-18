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
			}
		)+
	};
}
pub use impl_incrementable;

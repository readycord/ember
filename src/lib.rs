use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(test)]
mod tests;

pub mod wasm;
pub use wasm::*;

/// May 1st 2023 00:00. This is the beginning of time for Ember.
pub const EMBER_EPOCH: u128 = 1_682_899_200_000;

#[repr(C)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmberID(packed::PackedEmberID);

impl EmberID {
	/// Generates a new [`EmberID`].
	pub const fn new(/* ??? */) -> Self {
		todo!();
	}

	/// Milliseconds that have elapsed since the [`EMBER_EPOCH`].
	pub const fn timestamp(&self) -> u64 {
		todo!();
	}

	/// The node this [`EmberID`] belongs to.
	pub const fn node_id(&self) -> u16 {
		todo!();
	}

	pub const fn sequence(&self) -> u16 {
		todo!();
	}

	pub const fn magic(&self) -> u8 {
		todo!();
	}
}

impl From<u64> for EmberID {
	fn from(value: u64) -> Self {
		todo!();
	}
}

impl From<i64> for EmberID {
	fn from(value: i64) -> Self {
		todo!();
	}
}

impl From<EmberID> for u64 {
	fn from(ember_id: EmberID) -> Self {
		todo!();
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for EmberID {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		u64::from(*self).serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EmberID {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		u64::deserialize(deserializer).map(Into::into)
	}
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for EmberID {
	fn decode(
		value: <sqlx::Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
	) -> Result<Self, sqlx::error::BoxDynError> {
		<i64 as sqlx::Decode<sqlx::Postgres>>::decode(value).map(Into::into)
	}
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for EmberID {
	fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
		sqlx::postgres::PgTypeInfo::with_name("INT8")
	}
}

#[allow(dead_code)]
mod packed {
	use modular_bitfield::{
		bitfield,
		specifiers::{B11, B3, B41, B9},
	};

	#[bitfield]
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub(crate) struct PackedEmberID {
		pub(crate) timestamp: B41,
		pub(crate) node_id: B9,
		pub(crate) sequence: B11,
		pub(crate) magic: B3,
	}
}

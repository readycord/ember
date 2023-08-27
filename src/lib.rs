use {
	std::time::{SystemTime, UNIX_EPOCH},
	wasm_bindgen::prelude::wasm_bindgen,
};

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
	pub fn new(timestamp: u64, node_id: u16, sequence: u16, magic: u8) -> Self {
		Self(
			packed::PackedEmberID::new()
				.with_timestamp(timestamp)
				.with_node_id(node_id)
				.with_sequence(sequence)
				.with_magic(magic),
		)
	}

	/// Milliseconds that have elapsed since the [`EMBER_EPOCH`].
	pub fn timestamp(&self) -> u64 {
		self.0.timestamp() as u64
	}

	/// The node this [`EmberID`] belongs to.
	pub fn node_id(&self) -> u16 {
		self.0.node_id() as u16
	}

	pub fn sequence(&self) -> u16 {
		self.0.sequence() as u16
	}

	pub fn magic(&self) -> u8 {
		self.0.magic() as u8
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmberFactory {
	node_id: u16,
	sequence: u16,
	last_timestamp: u128,
}

impl EmberFactory {
	pub fn new(node_id: u16) -> Self {
		Self {
			node_id,
			sequence: 0,
			last_timestamp: SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.expect("Time went backwards")
				.as_millis() - EMBER_EPOCH,
		}
	}

	pub fn generate(&mut self, magic: u8) -> EmberID {
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("Time went backwards")
			.as_millis() - EMBER_EPOCH;

		let ember = EmberID(
			packed::PackedEmberID::new()
				.with_timestamp(timestamp as u64)
				.with_node_id(self.node_id)
				.with_sequence(self.sequence)
				.with_magic(magic),
		);

		// sequence resets every ms, so we need to increment it here and if a new ms has started,
		// reset it
		self.sequence += 1;
		if self.sequence >= 1023 {
			self.sequence = 0;
		}
		// timestamp increases by MS
		if timestamp > self.last_timestamp {
			self.sequence = 0;
		}

		self.last_timestamp = timestamp;
		ember
	}
}

impl From<u64> for EmberID {
	fn from(value: u64) -> Self {
		Self(
			packed::PackedEmberID::new()
				.with_timestamp(value as u64)
				.with_node_id(0)
				.with_sequence(0)
				.with_magic(0),
		)
	}
}

impl From<i64> for EmberID {
	fn from(value: i64) -> Self {
		Self(
			packed::PackedEmberID::new()
				.with_timestamp(value as u64)
				.with_node_id(0)
				.with_sequence(0)
				.with_magic(0),
		)
	}
}

impl From<EmberID> for u64 {
	fn from(ember_id: EmberID) -> Self {
		// get the bits from the packed struct
		let packed = ember_id.0.into_bytes();
		let mut value = 0;
		for (i, byte) in packed.iter().enumerate() {
			value |= (*byte as u64) << (i * 8);
		}
		value
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

#![allow(unused)] // TODO: remove
use {
	modular_bitfield::prelude::*,
	std::{
		fmt,
		time::{SystemTime, UNIX_EPOCH},
	},
};

const EMBER_EPOCH: u128 = 1_682_899_200_000;

#[bitfield]
struct PackedEmberID {
	timestamp: B41,
	node_id: B9,
	sequence: B11,
	magic: B3,
}

union EmberID {
	id: std::mem::ManuallyDrop<PackedEmberID>,
	ember: u64,
}

impl fmt::Display for EmberID {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { unsafe { write!(f, "{}", self.ember) } }
}

#[derive(Debug)]
struct UnpackedEmberID {
	timestamp: u64,
	unix_timestamp: u128,
	node_id: u16,
	sequence: u16,
	magic: u8,
}

struct EmberIDGenerator {
	node_id: u16,
	sequence: u16,
	last_timestamp: u128,
}

impl EmberIDGenerator {
	fn new(node_id: u16) -> Self {
		Self {
			node_id,
			sequence: 0,
			last_timestamp: SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.expect("Time went backwards")
				.as_millis() - EMBER_EPOCH,
		}
	}

	fn next(&mut self, magic: u8) -> EmberID {
		let timestamp =
			SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
				- EMBER_EPOCH;

		let mut ember = EmberID {
			id: std::mem::ManuallyDrop::new(
				PackedEmberID::new()
					.with_timestamp(timestamp as u64)
					.with_node_id(self.node_id as u16)
					.with_sequence(self.sequence as u16)
					.with_magic(magic),
			),
		};

		// sequence resets every ms, so we need to increment it here and if a new ms has started,
		// reset it
		self.sequence += 1;
		if self.sequence >= 1023 {
			self.sequence = 0;
		}
		// timestamp increases by MS
		if timestamp >= self.last_timestamp {
			self.sequence = 0;
		}

		self.last_timestamp = timestamp;
		ember
	}
}

fn decode_ember_id(ember: u64) -> UnpackedEmberID {
	let mut ember = EmberID {
		ember,
	};
	unsafe {
		UnpackedEmberID {
			timestamp: ember.id.timestamp(),
			unix_timestamp: ember.id.timestamp() as u128 + EMBER_EPOCH,
			node_id: ember.id.node_id(),
			sequence: ember.id.sequence(),
			magic: ember.id.magic(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ember() {
		let mut generator = EmberIDGenerator::new(0);
		let x = generator.next(0);
		unsafe {
			let unpacked = decode_ember_id(x.ember);
			println!("{}", x);
			println!("{:?}", unpacked);
		}
	}
}

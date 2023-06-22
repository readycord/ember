use modular_bitfield::prelude::*;

const EMBER_EPOCH: u64 = 1_682_899_200;

#[bitfield]
struct PackedEmberID {
	timestamp: B41,
	node_id: B9,
	sequence: B11,
	magic: B3,
}

struct EmberID {
	id: PackedEmberID,
}

impl EmberID {
	fn new() -> Self {
		// take unix time now and subtract it from the epoch
		// this will give us the number of seconds since the epoch
		Self {
			id: PackedEmberID::new()
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ember() {
    }
}

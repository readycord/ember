use {crate::EmberID, wasm_bindgen::prelude::wasm_bindgen};

#[wasm_bindgen]
pub fn decode(ember_id: u64) -> EmberID {
	todo!();
}

#[wasm_bindgen]
pub fn timestamp(ember_id: &EmberID) -> u64 {
	ember_id.timestamp()
}

#[wasm_bindgen]
pub fn node_id(ember_id: &EmberID) -> u16 {
	ember_id.node_id()
}

#[wasm_bindgen]
pub fn sequence(ember_id: &EmberID) -> u16 {
	ember_id.sequence()
}

#[wasm_bindgen]
pub fn magic(ember_id: &EmberID) -> u8 {
	ember_id.magic()
}

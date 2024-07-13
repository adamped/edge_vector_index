use wasm_bindgen::prelude::wasm_bindgen;

use crate::{EdgeVectorIndex, Index};

#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmEdgeVectorIndex {
    instance: EdgeVectorIndex,
}

#[wasm_bindgen]
impl WasmEdgeVectorIndex {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        WasmEdgeVectorIndex { instance: EdgeVectorIndex::new() }
    }

    pub fn add_to_index(&mut self, initial_data: Vec<f32>, metadata: String) {
        self.instance.index.push(Index { vectors: initial_data, metadata });
    }

    pub fn find_closest_match(&self, vector: &[f32]) -> String {
        match self.instance.find_closest_match(vector) {
            Some(index) => index.metadata.clone(),
            None => String::new(),
        }
    }

}

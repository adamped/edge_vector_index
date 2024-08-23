use similarity::cosine_similarity;

pub mod interop;
pub mod interop_wasm;
pub mod similarity;

#[derive(Clone)]
pub struct Index {
    pub vectors: Vec<f32>,
    pub metadata: String,
}

impl Index {
    pub fn new(vectors: Vec<f32>, metadata: String) -> Self {
        Index { vectors, metadata }
    }
}

#[derive(Clone)]
pub struct EdgeVectorIndex {
    index: Vec<Index>,
}

impl Default for EdgeVectorIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl EdgeVectorIndex {
    pub fn new() -> Self {
        EdgeVectorIndex { index: Vec::new() }
    }

    pub fn add_to_index(&mut self, initial_data: Vec<Index>) {
        self.index.extend(initial_data);
    }

    pub fn find_closest_match(&self, vector: &[f32]) -> Option<&Index> {
        let mut cosine: f32 = 0.0;
        let mut index_ref: Option<&Index> = None;

        for item in &self.index {
            let similarity = cosine_similarity(vector, &item.vectors);

            if similarity >= cosine {
                cosine = similarity;
                index_ref = Some(item);
            }
        }

        index_ref
    }
}

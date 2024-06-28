#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(slice_as_chunks)]
use std::simd::f32x4;
use std::simd::num::*;
use std::simd::*;

#[derive(Debug)]
pub struct Index {
    pub vectors: Vec<f32>,
    pub metadata: String,
}

impl Index {
    pub fn new(vectors: Vec<f32>, metadata: String) -> Self {
        Index { vectors, metadata }
    }
}

pub struct EdgeVectorIndex {
    index: Vec<Index>,
}

impl EdgeVectorIndex {
    pub fn new() -> Self {
        EdgeVectorIndex { index: Vec::new() }
    }

    pub fn init(&mut self, initial_data: Vec<Index>) {
        self.index.extend(initial_data);
    }

    pub fn find_closest_match(&self, vector: &[f32]) -> Option<&Index> {
        
        // Vectors must be divisible by 4 for SIMD
        assert_eq!(vector.len() % 4, 0);
        
        let mut cosine: f32 = 0.0;
        let mut index_ref: Option<&Index> = None;

        for item in &self.index {
            let similarity = self.cosine_similarity(vector, &item.vectors);

            if similarity > cosine {
                cosine = similarity;
                index_ref = Some(item);
            }
        }

        return index_ref;
    }

    fn cosine_similarity(&self, vector1: &[f32], vector2: &[f32]) -> f32 {

        assert_eq!(vector1.len(), vector2.len());

        let dot_product_value = Self::dot_product(vector1, vector2);
        let sum_sq1 = Self::dot_product(vector1, vector1);
        let sum_sq2 = Self::dot_product(vector2, vector2);

        let denominator = f32::sqrt(sum_sq1) * f32::sqrt(sum_sq2);

        return dot_product_value / denominator;
    }

    pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
              
        // Chunk the arrays to SIMD vector size
        a.array_chunks()
            .map(|&a| f32x4::from_array(a))
            .zip(b.array_chunks().map(|&b| f32x4::from_array(b)))
            .fold(f32x4::splat(0.0), |acc, (a, b)| a.mul_add(b, acc))
            .reduce_sum()
      }
}

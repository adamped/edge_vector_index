pub fn cosine_similarity(vector1: &[f32], vector2: &[f32]) -> f32 {
    assert_eq!(vector1.len(), vector2.len());

    let dot_product_value = dot_product(vector1, vector2);
    let sum_sq1 = dot_product(vector1, vector1);
    let sum_sq2 = dot_product(vector2, vector2);

    let magnitude = f32::sqrt(sum_sq1) * f32::sqrt(sum_sq2);

    if magnitude == 0.0 {
        // With 0 magnitude the relationship is undefined
        return f32::NAN;
    }

    dot_product_value / magnitude
}

fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

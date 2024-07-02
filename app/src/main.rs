use edge_vector_index::{self, EdgeVectorIndex, Index};

fn main() {
    let mut index = EdgeVectorIndex::new();

    index.add_to_index(vec![
        Index::new(vec![0.0, 0.1, 0.2, 0.3], String::from("0")),
        Index::new(vec![0.5, 0.5, 0.5, 0.5], String::from("1")),
    ]);

    let result = index.find_closest_match(&[0.5, 0.5, 0.5, 0.5]);

    println!("{}", result.unwrap().metadata);
}

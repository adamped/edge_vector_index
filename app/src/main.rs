use edge_vector_index::{self, Index, EdgeVectorIndex};

fn main() {
    let mut index = EdgeVectorIndex::new();

    index.init(vec![Index::new(vec![0.0, 0.1, 0.2, 0.3], String::from("Close")), Index::new(vec![0.5, 0.5, 0.5, 0.5], String::from("Far"))]);

    let result = index.find_closest_match(&[0.5, 0.5, 0.5, 0.5]);

    println!("{}", result.unwrap().metadata);
}



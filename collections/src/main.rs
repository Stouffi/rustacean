mod exercises;
mod hash_map;
mod string;
mod vector;

fn main() {
    // vector::vector();
    // string::string();
    // hash_map::hash_map();
    let stats = exercises::stats(vec![14, 14, 10, 19, 7, 9, 10, 10, 8, 7, 12, 15, 14, 14]);
    println!("{:#?}", stats)
}

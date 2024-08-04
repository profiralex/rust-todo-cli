mod repository;
mod generated;


fn main() {
    let _ = generated::models::Todo{..Default::default()};
    println!("Hello, world!");
}

fn main() {
    let crabby_treasure = String::from("gold coins");
    let borrow_treasure = &crabby_treasure;

    println!("Borrowed treasure: {}", borrow_treasure);
    println!("Original treasure: {}", crabby_treasure);
}

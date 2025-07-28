fn main() {
    let score: i32 = 0;
    println!("Initial score: {}", score);

    let score: i32 = score + 10;
    println!("Updated score: {}", score);

    let mut bonus: i32 = 5;
    bonus = bonus + 3;

    println!("Final score with bonus: {}", score + bonus);

    let name = String::from("Horry");
    println!("Player: {}", name);

}

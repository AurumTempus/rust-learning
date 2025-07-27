fn main() {
    
    let score: i32 = 0;
    println!("Initia score: {}", score);

    let new_score: i32 = score + 10;
    println!("Updated score: {}", new_score);

    let mut bonus = 5;
    bonus = bonus + 3;

    println!("Final score with bonus: {}", new_score + bonus);

    let name_user = String::from("Horry");
    println!("Player {}", name_user);

}

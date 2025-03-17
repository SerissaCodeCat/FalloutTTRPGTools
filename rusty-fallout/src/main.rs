use rand::{random_range, Rng};

fn main() {
    let goals = vec![
        "dice roller for loot table",
        "dice roller for location generation",
        "map generator for rolled location",
        "automatic sub-table rolling for loot",
        "generate random weapon without mod attachment",
        "generate random weapon WITH random mod attachment",
        "generate entirely random weapon",
        "generate random weapon within parameters",
        "same for armor tables",
        "gui",
        "player aids",
    ];
    println!("Hello, world!");
    println!("this will be the set of tools to assist GMs to run a Fallout TTRPG session.");
    println!("the goals for this project are to:");
    println!("{:?}", goals);

    let roll_test = random_range(1..20);
    println!("randomly rolled number is {}", roll_test);
}

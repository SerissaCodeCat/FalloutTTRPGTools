use rand::random_range;
use std::{io, string};

pub mod utility;
pub mod weapon;
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

    let mut roll_test = random_range(1..20);
    let mut roll_test2 = random_range(1..20);
    println!("randomly rolled D20 is {}", roll_test);
    roll_test = random_range(1..20);
    println!(
        "2D20 = {roll_test} & {roll_test2}, resulting in {}",
        roll_test2 + roll_test
    );

    loop {
        print!(
            "\n 
        1) LOOT TABLE \n 
        2) LOCATION GENERATOR \n 
        3) WEAPON LOOT GENERATOR \n 
        0) EXIT \n 
        \n 
        ENTER YOUR OPTION \n"
        );
        let menu_option = convert_input_to_int();
        match menu_option {
            1 => loot_table(),
            2 => location_generator(),
            3 => weapon::weapon_table_setup(),
            0 => exit_function(),
            _ => break,
        }
    }
}
fn loot_table() {
    print!("LOOT TABLE GOES HERE");
}
fn location_generator() {
    println!("LOCATION GENERATOR GOES HERE");
}
//fn weapon_table_setup() {
//    let mut weapon_rarity = 10;
//    println!("do you want a modified weapon? Y/N");
//    let modded_weapon = convert_input_to_bool();
//    println!("do you want to specify a rarity? Y/N");
//    let set_rarity = convert_input_to_bool();
//    if set_rarity {
//        print!("enter the maximum rarity level:");
//        weapon_rarity = convert_input_to_int();
//    }
//    basic_weapon_table(modded_weapon, weapon_rarity);
//}
//fn basic_weapon_table(create_modded_weapon: bool, create_weapon_of_rarity_level: i32) {
//    println!("BASIC WEAPON TABLE GOES HERE");
//    println!(
//        "select weapon rarity range: {}",
//        create_weapon_of_rarity_level.to_string();
//    );
//    println!("modify weapon: {}", create_modded_weapon.to_string());
//    if(create_modded_weapon){
//        weapon_mod_table(incoming);
//    }
//}
//fn random_weapon_of_rarity(incoming: i32) -> String{

//}
//fn weapon_mod_table(incoming: &String) {
//    println!("weapon mod goes here, after taking in the weapon type of {incoming}, and limmiting mods to availiable mods for said weapon");
//}
fn convert_input_to_bool() -> bool {
    loop {
        let mut tmp = String::new();
        io::stdin()
            .read_line(&mut tmp)
            .expect("failed to read line.");
        if tmp.trim() == "Y" || tmp.trim() == "y" {
            return true;
        } else if tmp.trim() == "N" || tmp.trim() == "n" {
            return false;
        } else {
            println!("please enter Y or N");
        };
    }
}
fn convert_input_to_int() -> i32 {
    let mut tmp = String::new();
    println!("please enter a number");
    io::stdin()
        .read_line(&mut tmp)
        .expect("could not read input.");
    return tmp.trim().parse().expect("please enter a number");
}
fn exit_function() {
    println!("data saving goes here, and then exit");
    std::process::exit(0);
}

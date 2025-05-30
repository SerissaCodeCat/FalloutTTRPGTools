use rand::random_range;
use std::{io, string};

pub mod energy_weapons_list;
pub mod small_guns_list;
pub mod utility;
pub mod weapon;
fn main() {
    println!("Hello, Overseer");
    println!("this set of tools is designed to help you run a Fallout TTRPG session.");

    loop {
        print!(
            "\n 
        1) WEAPON LOOT GENERATOR \n
        2) SHOP WEAPON INVENTORY GENERATOR (under development) \n
        3) PLAYER WEAPON PLANNER (under development) \n
        4) LOCATION & MAP GENERATOR (under development) \n,
        0) EXIT \n 
        \n 
        ENTER YOUR OPTION \n"
        );
        let menu_option = convert_input_to_int();
        match menu_option {
            1 => weapon_loot_generator(),
            2 => 
            3 => weapon::weapon_table_setup(),
            0 => exit_function(),
            _ => break,
        }
    }
}
fn weapon_loot_generator() {
    println!("LOOT TABLE GOES HERE");
}
fn shop_weapon_inventory_generator(){
    println!("under development");
}
fn location_and_map_generator() {
    println!("under development.");
}
fn player_weapon_planner() {
    println!("under development");
}
//fn convert_input_to_bool() -> bool {
//    loop {
//        let mut tmp = String::new();
//        io::stdin()
//            .read_line(&mut tmp)
//            .expect("failed to read line.");
//        if tmp.trim() == "Y" || tmp.trim() == "y" {
//            return true;
//        } else if tmp.trim() == "N" || tmp.trim() == "n" {
//            return false;
//        } else {
//            println!("please enter Y or N");
//        };
//    }
//}
//fn convert_input_to_int() -> i32 {
//    let mut tmp = String::new();
//    println!("please enter a number");
//    io::stdin()
//        .read_line(&mut tmp)
//        .expect("could not read input.");
//    return tmp.trim().parse().expect("please enter a number");
//}
//fn exit_function() {
//    println!("data saving goes here, and then exit");
//    std::process::exit(0);
//}

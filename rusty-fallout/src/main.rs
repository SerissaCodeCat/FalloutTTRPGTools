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
1) WEAPON LOOT GENERATOR
2) SHOP WEAPON INVENTORY GENERATOR (under development)
3) PLAYER WEAPON PLANNER (under development)
4) LOCATION & MAP GENERATOR (under development)
0) EXIT
         
ENTER YOUR OPTION \n"
        );
        let menu_option = utility::convert_input_to_int();
        match menu_option {
            1 => weapon_loot_generator(),
            2 => shop_weapon_inventory_generator(),
            3 => player_weapon_planner(),
            4 => location_and_map_generator(),
            0 => exit_function(),
            _ => break,
        }
    }
}
fn weapon_loot_generator() {
    let mut gun_list = small_guns_list::create_small_gun_list();
    gun_list.append(&mut energy_weapons_list::create_energy_weapon_list());
    let random_weapon = random_range(0..gun_list.len());
    print!("{}", gun_list[random_weapon]);
}
fn shop_weapon_inventory_generator() {
    println!("under development");
}
fn player_weapon_planner() {
    println!("under development")
}
fn location_and_map_generator() {
    println!("under development.");
}
fn exit_function() {
    println!("data saving goes here, and then exit");
    std::process::exit(0);
}

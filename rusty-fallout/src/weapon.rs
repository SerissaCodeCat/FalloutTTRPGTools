use std::{io, string};

pub enum WeaponType {
    SmallGun,
    BigGun,
    Energy,
    Explosive,
    Melee,
    Unarmed,
    Thowing,
}
pub enum DamageType {
    Physical,
    Energy,
    Radiation,
    Poison,
}
pub enum Range {
    Close,
    Medium,
    Long,
    Extream,
}
//pub enum AmmoType {
//    .38,
//    .308,
//    .45,
//    .50,
//    10mm,
//}
pub struct Properties {
    Accurate: bool,
    Blast: bool,
    CloseQuarters: bool,
    Concealed: bool,
    Debilitating: bool,
    Gattling: bool,
    Inaccurate: bool,
    Mine: bool,
    Nightvision: bool,
    Parry: bool,
    Recon: bool,
    Reliable: bool,
    Suppressed: bool,
    Thrown: bool,
    TwoHanded: bool,
    Unreliable: bool,
}
pub struct DamageEffects {
    burst: bool,
    breaking: bool,
    persistant: bool,
    peircing: i8,
    radioactive: bool,
    spread: bool,
    stun: bool,
    vicious: bool,
}
pub struct Weapon {
    name: String,
    rarity: i8,
    value: i32,
    weapon_type: WeaponType,
    damage_rating: i8,
    damage_type: DamageType,
    fire_rate: i8,
    properties: Properties,
    weight: f32,
}
pub fn weapon_table_setup() {
    let mut weapon_rarity = 10;
    println!("do you want a modified weapon? Y/N");
    let modded_weapon = crate::utility::convert_input_to_bool();
    println!("do you want to specify a rarity? Y/N");
    let set_rarity = crate::utility::convert_input_to_bool();
    if set_rarity {
        print!("enter the maximum rarity level:");
        weapon_rarity = crate::utility::convert_input_to_int();
    }
    basic_weapon_table(modded_weapon, weapon_rarity);
}
pub fn basic_weapon_table(create_modded_weapon: bool, create_weapon_of_rarity_level: i32) {
    println!("BASIC WEAPON TABLE GOES HERE");
    println!(
        "select weapon rarity range: {}",
        create_weapon_of_rarity_level.to_string()
    );
    println!("modify weapon: {}", create_modded_weapon.to_string());
    //if (create_modded_weapon) {
    //    weapon_mod_table(incoming);
    //}
}
pub fn random_weapon_of_rarity(incoming: i32) -> String {
    let mut return_val = String::from("a weapon of rarity value:").to_owned();
    return_val.push_str(&incoming.to_string());
    return return_val;
}
pub fn weapon_mod_table(incoming: &String) {
    println!("weapon mod goes here, after taking in the weapon type of {incoming}, and limmiting mods to availiable mods for said weapon");
}

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
pub struct Properties {
    accurate: bool,
    blast: bool,
    close_quarters: bool,
    concealed: bool,
    debilitating: bool,
    gattling: bool,
    inaccurate: bool,
    mine: bool,
    nightvision: bool,
    parry: bool,
    recon: bool,
    reliable: bool,
    suppressed: bool,
    thrown: bool,
    twoHanded: bool,
    unreliable: bool,
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
    damage_effects: DamageEffects,
    fire_rate: i8,
    properties: Properties,
    weight: f32,
}

pub fn weapon_table_setup() {
    let AmmoType = vec![
        ".308",
        ".44",
        ".45",
        ".50",
        "10mm",
        "5mm",
        "5,56",
        "shotgun shell",
        "Missile",
        "2mm EC",
        "Flamer Fuel",
        "Fusion Cell",
        "Gamma Round",
        "Plasma Cartridge",
        "Fusion Core",
        "Mini Nuke",
        "Flare",
        "Syringer Ammo",
    ];
    let SmallGunList = vec![
        ".44 Pistol",
        "10mm Pistol",
        "Flare gun",
        "Assault Rifle",
        "Combat Rifle",
        "Gauss Rifle",
        "Hunting Rifle",
        "Submachine Gun",
        "Semi-automatic Shotgun",
        "Double Barreled Shotgun",
        "Pipe bolt-action Gun",
        "Pipe-Gun",
        "Pipe-Revolver",
        "Railway Rifle",
        "Syringer",
    ];
    let EnergyWeaponList = vec![
        "Institute Laser",
        "Laser Musket",
        "Laser Gun",
        "Plasma Gun",
        "Gamma Gun",
    ];
    let BigGuns = vec![
        "Fat Man",
        "Flamer",
        "Gatling Laser",
        "Heavy Incinerator",
        "Junk Jet",
        "Minigun",
        "Missile Launcher",
    ];

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

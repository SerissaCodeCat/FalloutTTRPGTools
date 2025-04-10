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
    two_handed: bool,
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
    ammunition: String,
    range: Range,
}

pub const AMMO_TYPE:[&str; 18] = [".308", ".44", ".45", ".50", "10mm", "5mm", "5,56", "shotgun shell", "Missile", "2mm EC", "Flamer Fuel", "Fusion Cell", "Gamma Round", "Plasma Cartridge", "Fusion Core", "Mini Nuke", "Flare", "Syringer Ammo" ];
    

pub fn weapon_table_setup() {
    let small_gun_list = [
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from(".44 Pistol"), 
            rarity:2, 
            value: 99, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:true, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },















//        "10mm Pistol",
//        "Flare gun",
//        "Assault Rifle",
//        "Combat Rifle",
//        "Gauss Rifle",
//        "Hunting Rifle",
//        "Submachine Gun",
//        "Semi-automatic Shotgun",
//        "Double Barreled Shotgun",
//        "Pipe bolt-action Gun",
//        "Pipe-Gun",
//        "Pipe-Revolver",
//        "Railway Rifle",
//        "Syringer",
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

    let MeleeWeapons = vec![
        "Sword",
        "Combat Knife",
        "Machete",
        "Ripper",
        "Shishkebab",
        "Switchblade",
        "Baseball bat",
        "Aluminium Bat",
        "Board",
        "Lead Pipe",
        "Pipe Wrench",
        "Pool Que",
        "Rolling Pin",
        "Baton",
        "Sledgehammer",
        "Super Sledge",
        "Tire Iron",
        "walking Cane",
        "Boxing Glove",
        "Deathclaw Gauntlet",
        "Brass Knuckles",
        "Power Fist"
    ]

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

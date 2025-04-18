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

pub const AMMO_TYPE:[&str; 20] = [".308", ".44", ".45", ".50", "10mm", "5mm", "5,56", "shotgun shell", "Missile", "2mm EC", "Flamer Fuel", "Fusion Cell", "Gamma Round", "Plasma Cartridge", "Fusion Core", "Mini Nuke", "Flare", "Syringer Ammo", ".38", "Railway Spike" ];
    

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
            name: String::from("10mm Pistol"), 
            rarity:1, 
            value: 50, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 4, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
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
                reliable:true,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:2, 
            weight:4.0, 
            ammunition:AMMO_TYPE[4].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from("Flare Gun"), 
            rarity:1, 
            value: 50, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 3, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:false, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:true,
                suppressed:false,
                thrown:false,
                two_handed:false,
                unreliable:false
            },
            fire_rate:0, 
            weight:2.0, 
            ammunition:AMMO_TYPE[17].to_string(),
            range: Range::Medium
        },
        Weapon{
            name: String::from("Assault Rifle"), 
            rarity:2, 
            value: 144, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 5, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: true, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:false, 
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
                two_handed:true,
                unreliable:false
            },
            fire_rate:2, 
            weight:13.0, 
            ammunition:AMMO_TYPE[6].to_string(),
            range: Range::Medium
        },
        Weapon{
            name: String::from("Combat Rifle"), 
            rarity:2, 
            value: 117, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 5, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:false, 
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
                two_handed:true,
                unreliable:false
            },
            fire_rate:2, 
            weight:11.0, 
            ammunition:AMMO_TYPE[2].to_string(),
            range: Range::Medium
        },
        Weapon{
            name: String::from("Gauss Rifle"), 
            rarity:4, 
            value: 228, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 10, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 1, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:false, 
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
                two_handed:true,
                unreliable:false
            },
            fire_rate:1, 
            weight:16.0, 
            ammunition:AMMO_TYPE[0].to_string(),
            range: Range::Medium
        },
        Weapon{
            name: String::from("Hunting Rifle"), 
            rarity:2, 
            value: 55, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 6, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 1, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
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
                two_handed:true,
                unreliable:false
            },
            fire_rate:0, 
            weight:10.0, 
            ammunition:AMMO_TYPE[1].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from("Submachine Gun"), 
            rarity:1, 
            value: 109, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 3, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: true, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:false, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:true,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:true,
                unreliable:false
            },
            fire_rate:3, 
            weight:12.0, 
            ammunition:AMMO_TYPE[2].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from("Automatic Shotgun"), 
            rarity:2, 
            value: 87, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 5, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: true, 
                stun: false,
                vicious: false
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:false, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:true,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:true,
                unreliable:false
            },
            fire_rate:2, 
            weight:11.0, 
            ammunition:AMMO_TYPE[7].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from("Double Barreled Shotgun"), 
            rarity:1, 
            value: 39, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 5, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: true, 
                stun: false,
                vicious: true
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:false, 
                concealed:false, 
                debilitating: false, 
                gattling:false,
                inaccurate:true,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:true,
                unreliable:false
            },
            fire_rate:0, 
            weight:9.0, 
            ammunition:AMMO_TYPE[7].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from("Pipe bolt-action"), 
            rarity:0, 
            value: 30, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 5, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 1, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:false, 
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
                unreliable:true
            },
            fire_rate:0, 
            weight:3.0, 
            ammunition:AMMO_TYPE[0].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from("Pipe Gun"), 
            rarity:0, 
            value: 30, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 3, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
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
                unreliable:true
            },
            fire_rate:2, 
            weight:2.0, 
            ammunition:AMMO_TYPE[18].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from("Pipe-Revolver"), 
            rarity:0, 
            value: 25, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 4, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
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
                unreliable:true
            },
            fire_rate:1, 
            weight:4.0, 
            ammunition:AMMO_TYPE[2].to_string(),
            range: Range::Close
        },
        Weapon{
            name: String::from("Railway Rifle"), 
            rarity:4, 
            value:290, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 10, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: true, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:false, 
                concealed:false, 
                debilitating: true, 
                gattling:false,
                inaccurate:false,
                mine:false,
                nightvision:false,
                parry:false,
                recon:false, 
                reliable:false,
                suppressed:false,
                thrown:false,
                two_handed:true,
                unreliable:true
            },
            fire_rate:0, 
            weight:14.0, 
            ammunition:AMMO_TYPE[19].to_string(),
            range: Range::Medium
        },
        Weapon{
            name: String::from("Syringer"), 
            rarity:2, 
            value: 132, 
            weapon_type: WeaponType::SmallGun, 
            damage_rating: 3, 
            damage_type: DamageType::Physical, 
            damage_effects: DamageEffects{
                burst: false, 
                breaking: false, 
                persistant: false, 
                peircing: 0, 
                radioactive: false, 
                spread: false, 
                stun: false,
                vicious: false
            },
            properties: Properties{
                accurate:false, 
                blast:false, 
                close_quarters:false, 
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
                two_handed:true,
                unreliable:false
            },
            fire_rate:0, 
            weight:6.0, 
            ammunition:AMMO_TYPE[17].to_string(),
            range: Range::Medium
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

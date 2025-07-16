use std::array;

use crate::weapon::*;

pub fn create_small_gun_list() -> Vec<Weapon> {
    let default_properties = Properties {
        accurate: false,
        blast: false,
        close_quarters: false,
        concealed: false,
        debilitating: false,
        gattling: false,
        inaccurate: false,
        mine: false,
        nightvision: false,
        parry: false,
        recon: false,
        reliable: false,
        suppressed: false,
        thrown: false,
        two_handed: false,
        unreliable: false,
    };
    let default_damage_effects = DamageEffects {
        burst: false,
        breaking: false,
        persistant: false,
        peircing: 0,
        radioactive: false,
        spread: false,
        stun: false,
        vicious: false,
    };
    return vec![
        Weapon {
            name: String::from(".44 Pistol"),
            rarity: 2,
            value: 99,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 6,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                vicious: true,
                ..default_damage_effects
            },
            properties: Properties {
                close_quarters: true,
                ..default_properties
            },
            fire_rate: 1,
            weight: 4.0,
            ammunition: AmmoType::Point44,
            range: Range::Close,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("10mm Pistol"),
            rarity: 1,
            value: 50,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 4,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            properties: Properties {
                close_quarters: true,
                ..default_properties
            },
            fire_rate: 2,
            weight: 4.0,
            ammunition: AmmoType::TenMillimeter,
            range: Range::Close,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Flare Gun"),
            rarity: 1,
            value: 50,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            properties: Properties {
                reliable: true,
                ..default_properties
            },
            fire_rate: 0,
            weight: 2.0,
            ammunition: AmmoType::Flare,
            range: Range::Medium,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Assault Rifle"),
            rarity: 2,
            value: 144,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 5,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                burst: true,
                ..default_damage_effects
            },
            properties: Properties {
                two_handed: true,
                ..default_properties
            },
            fire_rate: 2,
            weight: 13.0,
            ammunition: AmmoType::FivePointFiveSix,
            range: Range::Medium,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Combat Rifle"),
            rarity: 2,
            value: 117,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 5,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            properties: Properties {
                two_handed: true,
                ..default_properties
            },
            fire_rate: 2,
            weight: 11.0,
            ammunition: AmmoType::Point45,
            range: Range::Medium,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Gauss Rifle"),
            rarity: 4,
            value: 228,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 10,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            properties: Properties {
                two_handed: true,
                ..default_properties
            },
            fire_rate: 1,
            weight: 16.0,
            ammunition: AmmoType::TwoMilimeterEC,
            range: Range::Medium,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Hunting Rifle"),
            rarity: 2,
            value: 55,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 6,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            properties: Properties {
                close_quarters: true,
                two_handed: true,
                ..default_properties
            },
            fire_rate: 0,
            weight: 10.0,
            ammunition: AmmoType::Point308,
            range: Range::Close,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Submachine Gun"),
            rarity: 1,
            value: 109,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                burst: true,
                ..default_damage_effects
            },
            properties: Properties {
                inaccurate: true,
                two_handed: true,
                ..default_properties
            },
            fire_rate: 3,
            weight: 12.0,
            ammunition: AmmoType::Point45,
            range: Range::Close,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Automatic Shotgun"),
            rarity: 2,
            value: 87,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 5,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                spread: true,
                ..default_damage_effects
            },
            properties: Properties {
                inaccurate: true,
                two_handed: true,
                ..default_properties
            },
            fire_rate: 2,
            weight: 11.0,
            ammunition: AmmoType::ShotgunShell,
            range: Range::Close,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Double Barreled Shotgun"),
            rarity: 1,
            value: 39,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 5,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                spread: true,
                vicious: true,
                ..default_damage_effects
            },
            properties: Properties {
                inaccurate: true,
                two_handed: true,
                ..default_properties
            },
            fire_rate: 0,
            weight: 9.0,
            ammunition: AmmoType::ShotgunShell,
            range: Range::Close,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Pipe bolt-action"),
            rarity: 0,
            value: 30,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 5,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            properties: Properties {
                unreliable: true,
                ..default_properties
            },
            fire_rate: 0,
            weight: 3.0,
            ammunition: AmmoType::Point308,
            range: Range::Close,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Pipe Gun"),
            rarity: 0,
            value: 30,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            properties: Properties {
                close_quarters: true,
                unreliable: true,
                ..default_properties
            },
            fire_rate: 2,
            weight: 2.0,
            ammunition: AmmoType::Point38,
            range: Range::Close,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Pipe-Revolver"),
            rarity: 0,
            value: 25,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 4,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            properties: Properties {
                close_quarters: true,
                unreliable: true,
                ..default_properties
            },
            fire_rate: 1,
            weight: 4.0,
            ammunition: AmmoType::Point45,
            range: Range::Close,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Railway Rifle"),
            rarity: 4,
            value: 290,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 10,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                breaking: true,
                ..default_damage_effects
            },
            properties: Properties {
                debilitating: true,
                two_handed: true,
                unreliable: true,
                ..default_properties
            },
            fire_rate: 0,
            weight: 14.0,
            ammunition: AmmoType::RailwaySpike,
            range: Range::Medium,
            specialNotes: String::from(""),
        },
        Weapon {
            name: String::from("Syringer"),
            rarity: 2,
            value: 132,
            weapon_type: WeaponType::SmallGun,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            properties: Properties {
                two_handed: true,
                ..default_properties
            },
            fire_rate: 0,
            weight: 6.0,
            ammunition: AmmoType::SyringerAmmo,
            range: Range::Medium,
            specialNotes: String::from(""),
        },
    ];
}

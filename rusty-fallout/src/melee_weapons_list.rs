use std::vec;

use crate::weapon::*;
pub fn create_melee_weapon_list() -> Vec<Weapon> {
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
            name: String::from("Unarmed Strike"),
            rarity: 0,
            value: 0,
            weapon_type: WeaponType::Unarmed,
            damage_rating: 2,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 0.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Handy Rock"),
            rarity: 0,
            value: 0,
            weapon_type: WeaponType::Unarmed,
            damage_rating: 2,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                vicious: true,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                thrown: true,
                ..default_properties
            },
            weight: 1.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Gun Bash (1H)"),
            rarity: 0,
            value: 0,
            weapon_type: WeaponType::Melee,
            damage_rating: 2,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                stun: true,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 0.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Gun Bash (2H)"),
            rarity: 0,
            value: 0,
            weapon_type: WeaponType::Melee,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                stun: true,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 0.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Sword"),
            rarity: 2,
            value: 50,
            weapon_type: WeaponType::Melee,
            damage_rating: 4,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                parry: true,
                ..default_properties
            },
            weight: 3.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Combat Knife"),
            rarity: 1,
            value: 25,
            weapon_type: WeaponType::Melee,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 1.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Machete"),
            rarity: 1,
            value: 25,
            weapon_type: WeaponType::Melee,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 2.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Ripper"),
            rarity: 2,
            value: 50,
            weapon_type: WeaponType::Melee,
            damage_rating: 4,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                vicious: true,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 6.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Shishkebab"),
            rarity: 3,
            value: 200,
            weapon_type: WeaponType::Melee,
            damage_rating: 5,
            damage_type: DamageType::Energy,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                parry: true,
                ..default_properties
            },
            weight: 3.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Baseball Bat"),
            rarity: 1,
            value: 25,
            weapon_type: WeaponType::Melee,
            damage_rating: 4,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                two_handed: true,
                ..default_properties
            },
            weight: 3.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Switchblade"),
            rarity: 0,
            value: 20,
            weapon_type: WeaponType::Melee,
            damage_rating: 2,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 1.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Aluminum Baseball Bat"),
            rarity: 2,
            value: 32,
            weapon_type: WeaponType::Melee,
            damage_rating: 5,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                two_handed: true,
                ..default_properties
            },
            weight: 2.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Board"),
            rarity: 0,
            value: 20,
            weapon_type: WeaponType::Melee,
            damage_rating: 4,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                two_handed: true,
                ..default_properties
            },
            weight: 3.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Lead Pipe"),
            rarity: 0,
            value: 15,
            weapon_type: WeaponType::Unarmed,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 3.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Pipe Wrentch"),
            rarity: 1,
            value: 30,
            weapon_type: WeaponType::Melee,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 2.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Pool Cue"),
            rarity: 0,
            value: 10,
            weapon_type: WeaponType::Melee,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                two_handed: true,
                ..default_properties
            },
            weight: 1.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Rolling Pin"),
            rarity: 0,
            value: 10,
            weapon_type: WeaponType::Melee,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 1.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Baton"),
            rarity: 1,
            value: 15,
            weapon_type: WeaponType::Melee,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 2.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Sledgehammer"),
            rarity: 2,
            value: 40,
            weapon_type: WeaponType::Melee,
            damage_rating: 5,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 12.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Super Sledge"),
            rarity: 3,
            value: 180,
            weapon_type: WeaponType::Melee,
            damage_rating: 6,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                breaking: true,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                two_handed: true,
                ..default_properties
            },
            weight: 20.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Tire Iron"),
            rarity: 1,
            value: 25,
            weapon_type: WeaponType::Melee,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 2.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Walking Cane"),
            rarity: 0,
            value: 10,
            weapon_type: WeaponType::Unarmed,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 2.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Boxing Glove"),
            rarity: 1,
            value: 10,
            weapon_type: WeaponType::Unarmed,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                stun: true,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 1.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Deathclaw Gauntlet"),
            rarity: 3,
            value: 75,
            weapon_type: WeaponType::Unarmed,
            damage_rating: 5,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 10.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Knuckles"),
            rarity: 1,
            value: 10,
            weapon_type: WeaponType::Unarmed,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                concealed: true,
                ..default_properties
            },
            weight: 0.5,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Power Fist"),
            rarity: 2,
            value: 100,
            weapon_type: WeaponType::Unarmed,
            damage_rating: 4,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                stun: true,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                ..default_properties
            },
            weight: 4.0,
            ammunition: AmmoType::None,
            range: Range::Close,
            special_notes: String::from(""),
        },
    ];
}

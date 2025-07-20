use std::vec; 

use crate::weapon::*;
pub fn create_heavy_weapon_list() -> Vec<Weapon> {
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
        two_handed: true,
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
            name: String::from("Fat Man"),
            rarity: 4,
            value: 512,
            weapon_type: WeaponType::HeavyGun,
            damage_rating: 21,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                breaking: true,
                radioactive:true,
                vicious:true,                            
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                blast:true,
                inaccurate: true,
                ..default_properties
            },
            weight: 31.0,
            ammunition: AmmoType::MiniNuke,
            range: Range::Medium,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Flamer"),
            rarity: 3,
            value: 137,
            weapon_type: WeaponType::HeavyGun,
            damage_rating: 3,
            damage_type: DamageType::Energy,
            damage_effects: DamageEffects {
                burst: true,
                persistant:true,
                spread:true,                            
                ..default_damage_effects
            },
            fire_rate: 4,
            properties: Properties {
                debilitating:true,
                inaccurate: true,
                ..default_properties
            },
            weight: 16.0,
            ammunition: AmmoType::FlamerFuel,
            range: Range::Close,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Gattling Later"),
            rarity: 3,
            value: 804,
            weapon_type: WeaponType::HeavyGun,
            damage_rating: 3,
            damage_type: DamageType::Energy,
            damage_effects: DamageEffects {
                burst: true,
                peircing:1,                           
                ..default_damage_effects
            },
            fire_rate: 6,
            properties: Properties {
                gattling:true,
                inaccurate: true,
                ..default_properties
            },
            weight: 19.0,
            ammunition: AmmoType::FusionCore,
            range: Range::Medium,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Heavy Incinerator"),
            rarity: 4,
            value: 350,
            weapon_type: WeaponType::HeavyGun,
            damage_rating: 5,
            damage_type: DamageType::Energy,
            damage_effects: DamageEffects {
                burst: true,
                persistant:true,
                spread:true,                            
                ..default_damage_effects
            },
            fire_rate: 3,
            properties: Properties {
                debilitating:true,
                ..default_properties
            },
            weight: 20.0,
            ammunition: AmmoType::FlamerFuel,
            range: Range::Medium,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Junk Jet"),
            rarity: 3,
            value: 285,
            weapon_type: WeaponType::HeavyGun,
            damage_rating: 6,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {                         
                ..default_damage_effects
            },
            fire_rate: 1,
            properties: Properties {
               ..default_properties
            },
            weight: 30.0,
            ammunition: AmmoType::None,
            range: Range::Medium,       
            special_notes: String::from("When you fire the junk jet, choose one item from your inventory. that item is fired by the gun, and removed from your inventory. to fire a second, or further shot, choose another item to remove from your inventory."),
        },
        Weapon {
            name: String::from("MiniGun"),
            rarity: 2,
            value: 382,
            weapon_type: WeaponType::HeavyGun,
            damage_rating: 3,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                burst: true,
                spread:true,
                ..default_damage_effects
            },
            fire_rate: 5,
            properties: Properties {
                gattling: true,
                inaccurate: true,
                ..default_properties
            },
            weight: 27.0,
            ammunition: AmmoType::FiveMilimeter,
            range: Range::Medium,
            special_notes: String::from(""),
        },
        Weapon {
            name: String::from("Missile Launcher"),
            rarity: 4,
            value: 314,
            weapon_type: WeaponType::HeavyGun,
            damage_rating: 11,
            damage_type: DamageType::Physical,
            damage_effects: DamageEffects {
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                blast:true,
                ..default_properties
            },
            weight: 21.0,
            ammunition: AmmoType::Missile,
            range: Range::Long,
            special_notes: String::from(""),
        }

    ]
}
 

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
#[derive(PartialEq, Eq)]
pub enum AmmoType {
    None,
    Point308,
    Point44,
    Point45,
    Point50,
    TenMillimeter,
    FiveMilimeter,
    FivePointFiveSix,
    ShotgunShell,
    Missile,
    TwoMilimeterEC,
    FlamerFuel,
    FusionCell,
    GammaRound,
    PlasmaCartridge,
    FusionCore,
    MiniNuke,
    Flare,
    SyringerAmmo,
    Point38,
    RailwaySpike,
}

pub enum DamageType {
    Physical,
    Energy,
    EnergyAndPhysical,
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
    ammunition: AmmoType,
    range: Range,
}
impl Weapon {
    fn apply_hardened_mod(&mut self) -> bool {
        self.damage_rating = self.damage_rating + 2;
        return true;
    }
    fn apply_powerful_mod(&mut self) -> bool {
        self.damage_rating = self.damage_rating + 2;
        return true;
    }
    fn apply_advanced_mod(&mut self) -> bool {
        self.damage_rating = self.damage_rating + 3;
        self.fire_rate = self.fire_rate + 1;
        return true;
    }
    fn apply_calibrated_mod(&mut self) -> bool {
        if self.damage_effects.vicious != true {
            self.damage_effects.vicious = true;
            return true;
        } else {
            return false;
        }
    }
    fn apply_automatic_mod(&mut self) -> bool {
        if self.properties.inaccurate == true {
            return false;
        }
        if self.damage_effects.burst == true {
            return false;
        } else {
            self.damage_rating = self.damage_rating - 1;
            self.fire_rate = self.fire_rate + 2;
            self.properties.inaccurate = true;
            self.damage_effects.burst = true;
            return true;
        }
    }
    fn apply_hair_trigger_mod(&mut self) -> bool {
        self.fire_rate = self.fire_rate + 1;
        return true;
    }
    fn apply_point_38_mod(&mut self) -> bool {
        if self.ammunition != AmmoType::Point38 {
            self.damage_rating = 4;
            self.ammunition = AmmoType::Point38;
            return true;
        }
        return false;
    }
    fn apply_point_308_mod(&mut self) -> bool {
        if self.ammunition != AmmoType::Point308 {
            self.damage_rating = 9;
            self.ammunition = AmmoType::Point308;
            return true;
        }
        return false;
    }

    fn apply_point_45_mod(&mut self) -> bool {
        if self.ammunition != AmmoType::Point45 {
            self.damage_rating = 4;
            self.ammunition = AmmoType::Point45;
            self.fire_rate = self.fire_rate + 1;
            return true;
        }
        return false;
    }
    fn apply_point_50_mod(&mut self) -> bool {
        if self.ammunition != AmmoType::Point50 {
            self.damage_rating = 8;
            self.ammunition = AmmoType::Point50;
            self.damage_effects.vicious = true;
            return true;
        }
        return false;
    }
    fn apply_automatic_piston_mod(&mut self) -> bool {
        match self.range {
            Range::Close => return false,
            Range::Medium => self.range = Range::Close,
            Range::Long => self.range = Range::Medium,
            Range::Extream => self.range = Range::Long,
        }
        self.fire_rate = self.fire_rate + 2;
        return true;
    }

    fn apply_snubnosed_mod(&mut self) -> bool {
        if self.properties.accurate == true {
            self.properties.accurate = false;
        } else if self.properties.inaccurate == true {
            return false;
        } else {
            self.properties.inaccurate = true;
        }
        return true;
    }
    fn apply_bull_barrel_mod(&mut self) -> bool {
        if self.properties.unreliable == true {
            self.properties.unreliable = false;
        } else if self.properties.reliable == true {
            return false;
        } else {
            self.properties.reliable = true;
        }
        return true;
    }
}
pub fn weapon_table_setup() {
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
    let small_gun_list = [
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
        },
    ];
    let energy_weapon = vec![
        Weapon {
            name: String::from("Institute Laser"),
            rarity: 2,
            value: 50,
            weapon_type: WeaponType::Energy,
            damage_rating: 3,
            damage_type: DamageType::Energy,
            damage_effects: DamageEffects {
                burst: true,
                ..default_damage_effects
            },
            fire_rate: 3,
            properties: Properties {
                close_quarters: true,
                inaccurate: true,
                ..default_properties
            },
            weight: 4.0,
            ammunition: AmmoType::FusionCell,
            range: Range::Close,
        },
        Weapon {
            name: String::from("Laser Musket"),
            rarity: 1,
            value: 57,
            weapon_type: WeaponType::Energy,
            damage_rating: 5,
            damage_type: DamageType::Energy,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            fire_rate: 0,
            properties: Properties {
                two_handed: true,
                ..default_properties
            },
            weight: 13.0,
            ammunition: AmmoType::FusionCell,
            range: Range::Medium,
        },
        Weapon {
            name: String::from("Laser Gun"),
            rarity: 2,
            value: 69,
            weapon_type: WeaponType::Energy,
            damage_rating: 4,
            damage_type: DamageType::Energy,
            damage_effects: DamageEffects {
                peircing: 1,
                ..default_damage_effects
            },
            fire_rate: 3,
            properties: Properties {
                close_quarters: true,
                ..default_properties
            },
            weight: 4.0,
            ammunition: AmmoType::FusionCell,
            range: Range::Close,
        },
        Weapon {
            name: String::from("Plasma Gun"),
            rarity: 3,
            value: 123,
            weapon_type: WeaponType::Energy,
            damage_rating: 3,
            damage_type: DamageType::EnergyAndPhysical,
            damage_effects: DamageEffects {
                burst: true,
                ..default_damage_effects
            },
            fire_rate: 3,
            properties: Properties {
                close_quarters: true,
                inaccurate: true,
                ..default_properties
            },
            weight: 4.0,
            ammunition: AmmoType::PlasmaCartridge,
            range: Range::Close,
        },
        Weapon {
            name: String::from("Gamma Gun"),
            rarity: 5,
            value: 156,
            weapon_type: WeaponType::Energy,
            damage_rating: 3,
            damage_type: DamageType::Radiation,
            damage_effects: DamageEffects {
                peircing: 1,
                stun: true,
                ..default_damage_effects
            },
            fire_rate: 3,
            properties: Properties {
                blast: true,
                inaccurate: true,
                ..default_properties
            },
            weight: 3.0,
            ammunition: AmmoType::GammaRound,
            range: Range::Close,
        },
    ];
    let big_guns = vec![
        "Fat Man",
        "Flamer",
        "Gatling Laser",
        "Heavy Incinerator",
        "Junk Jet",
        "Minigun",
        "Missile Launcher",
    ];

    let melee_weapons = vec![
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
        "Power Fist",
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

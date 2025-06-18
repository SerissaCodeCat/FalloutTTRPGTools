use std::fmt;
use std::{io, string};

#[derive(PartialEq, Eq, Debug)]
pub enum WeaponType {
    SmallGun,
    BigGun,
    Energy,
    Explosive,
    Melee,
    Unarmed,
    Thowing,
}

impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        //write!(f, "Type: {}", self.weapon_type)
    }
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

#[derive(PartialEq, Eq, Debug)]
pub enum DamageType {
    Physical,
    Energy,
    EnergyAndPhysical,
    Radiation,
    Poison,
}
impl fmt::Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        //write!(f, "Type: {}", self.weapon_type)
    }
}

#[derive(PartialEq, Eq)]
pub enum Range {
    Close,
    Medium,
    Long,
    Extream,
}

pub struct Properties {
    pub accurate: bool,
    pub blast: bool,
    pub close_quarters: bool,
    pub concealed: bool,
    pub debilitating: bool,
    pub gattling: bool,
    pub inaccurate: bool,
    pub mine: bool,
    pub nightvision: bool,
    pub parry: bool,
    pub recon: bool,
    pub reliable: bool,
    pub suppressed: bool,
    pub thrown: bool,
    pub two_handed: bool,
    pub unreliable: bool,
}

pub struct DamageEffects {
    pub burst: bool,
    pub breaking: bool,
    pub persistant: bool,
    pub peircing: i8,
    pub radioactive: bool,
    pub spread: bool,
    pub stun: bool,
    pub vicious: bool,
}
pub struct Weapon {
    pub name: String,
    pub rarity: i8,
    pub value: i32,
    pub weapon_type: WeaponType,
    pub damage_rating: i8,
    pub damage_type: DamageType,
    pub damage_effects: DamageEffects,
    pub fire_rate: i8,
    pub properties: Properties,
    pub weight: f32,
    pub ammunition: AmmoType,
    pub range: Range,
    pub specialNotes: String,
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}\nType: {}\nDamage Rating: {}\nDamage Type: {}",
            self.name, self.weapon_type, self.damage_rating, self.damage_type
        )
        //write!(f, "Type: {}", self.weapon_type)
    }
}
impl Weapon {
    /////////////////
    //RECIEVER MODS//
    /////////////////
    fn apply_hardened_reciever_mod(&mut self) -> bool {
        self.damage_rating += 1;
        self.value += 20;
        return true;
    }
    fn apply_powerful_reciever_mod(&mut self) -> bool {
        self.damage_rating += 2;
        self.weight += 1.0;
        self.value += 25;
        return true;
    }
    fn apply_advanced_reciever_mod(&mut self) -> bool {
        self.damage_rating = self.damage_rating + 3;
        self.fire_rate += 1;
        self.weight += 2.0;
        self.value += 35;
        return true;
    }
    fn apply_calibrated_reciever_mod(&mut self) -> bool {
        if self.damage_effects.vicious != true {
            self.damage_effects.vicious = true;
            self.value += 25;
            return true;
        } else {
            return false;
        }
    }
    fn apply_automatic_reciever_mod(&mut self) -> bool {
        if self.properties.inaccurate == true {
            return false;
        }
        if self.damage_effects.burst == true {
            return false;
        } else {
            self.damage_rating -= 1;
            self.fire_rate += 2;
            self.properties.inaccurate = true;
            self.damage_effects.burst = true;
            self.weight += 1.0;
            self.value += 30;
            return true;
        }
    }
    fn apply_hair_trigger_reciever_mod(&mut self) -> bool {
        self.fire_rate += 1;
        self.value += 20;
        return true;
    }
    fn apply_point_38_reciever_mod(&mut self) -> bool {
        if self.ammunition != AmmoType::Point38 {
            self.damage_rating = 4;
            self.ammunition = AmmoType::Point38;
            self.weight += 3.0;
            self.value += 20;
            return true;
        }
        return false;
    }
    fn apply_point_308_reciever_mod(&mut self) -> bool {
        if self.ammunition != AmmoType::Point308 {
            self.damage_rating = 9;
            self.ammunition = AmmoType::Point308;
            self.weight += 4.0;
            self.value += 40;
            return true;
        }
        return false;
    }

    fn apply_point_45_reciever_mod(&mut self) -> bool {
        if self.ammunition != AmmoType::Point45 {
            self.damage_rating = 4;
            self.ammunition = AmmoType::Point45;
            self.fire_rate += 1;
            self.weight += 2.0;
            self.value += 19;
            return true;
        }
        return false;
    }
    fn apply_point_50_reciever_mod(&mut self) -> bool {
        if self.ammunition != AmmoType::Point50 {
            self.damage_rating = 8;
            self.ammunition = AmmoType::Point50;
            self.damage_effects.vicious = true;
            self.weight += 4.0;
            self.value += 30;
            return true;
        }
        return false;
    }
    fn apply_automatic_piston_reciever_mod(&mut self) -> bool {
        match self.range {
            Range::Close => return false,
            Range::Medium => self.range = Range::Close,
            Range::Long => self.range = Range::Medium,
            Range::Extream => self.range = Range::Long,
        }
        self.fire_rate += 2;
        self.value += 75;
        self.weight += 2.0;
        return true;
    }

    ////////////////
    //BARREL MODS //
    ////////////////
    fn apply_snubnosed_barrel_mod(&mut self) -> bool {
        if self.properties.inaccurate == true {
            return false;
        } else if self.properties.accurate == true {
            self.properties.accurate = false;
        } else {
            self.properties.inaccurate = true;
        }
        self.weight -= 1.0;
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
        self.value += 10;
        return true;
    }
    fn apply_long_barrel_mod(&mut self) -> bool {
        match self.range {
            Range::Close => self.range = Range::Medium,
            Range::Medium => self.range = Range::Long,
            Range::Long => self.range = Range::Extream,
            Range::Extream => return false,
        }
        self.weight += 1.0;
        self.value += 20;
        return true;
    }
    fn apply_ported_barrel_mod(&mut self) -> bool {
        match self.range {
            Range::Close => self.range = Range::Medium,
            Range::Medium => self.range = Range::Long,
            Range::Long => self.range = Range::Extream,
            Range::Extream => return false,
        }
        self.weight += 1.0;
        self.fire_rate += 1;
        self.value += 35;
        return true;
    }
    fn apply_vented_barrel_mod(&mut self) -> bool {
        if self.properties.reliable == true {
            return false;
        } else if self.properties.unreliable == true {
            self.properties.unreliable = false;
        } else {
            self.properties.reliable = true;
        }
        match self.range {
            Range::Close => self.range = Range::Medium,
            Range::Medium => self.range = Range::Long,
            Range::Long => self.range = Range::Extream,
            Range::Extream => return false,
        }
        self.fire_rate += 1;
        self.weight += 1.0;
        self.value += 36;
        return true;
    }
    fn apply_sawn_off_mod(&mut self) -> bool {
        if self.properties.two_handed == false {
            return false;
        } else if self.properties.close_quarters == true {
            return false;
        }
        self.properties.two_handed = true;
        self.properties.close_quarters = true;
        self.weight -= 2.0;
        self.value += 3;
        return true;
    }
    fn apply_finned_barrel_mod(&mut self) -> bool {
        match self.range {
            Range::Close => self.range = Range::Medium,
            Range::Medium => self.range = Range::Long,
            Range::Long => self.range = Range::Extream,
            Range::Extream => return false,
        }
        self.damage_rating += 1;
        self.value += 15;
        self.weight += 2.0;
        return true;
    }

    ///////////////
    // GRIP MODS //
    ///////////////
    fn apply_comfort_grip_mod(&mut self) -> bool {
        if self.properties.inaccurate != true {
            return false;
        }
        self.properties.inaccurate = false;
        self.value += 6;
        return true;
    }
    fn apply_sharpshooters_grip_mod(&mut self) -> bool {
        if self.properties.inaccurate != true {
            return false;
        }
        self.properties.inaccurate = false;
        self.damage_effects.peircing += 1;
        self.value += 10;
        return true;
    }

    ////////////////
    // STOCK MODS //
    ////////////////
    fn apply_full_stock_mod(&mut self) -> bool {
        if self.properties.two_handed == true {
            return false;
        }
        if self.properties.inaccurate == false {
            return false;
        }
        self.properties.two_handed = true;
        self.properties.inaccurate = false;
        self.weight += 1.0;
        self.value += 10;
        return true;
    }
    fn apply_marksmans_stock_mod(&mut self) -> bool {
        if self.properties.two_handed == true {
            return false;
        }
        if self.properties.accurate == true {
            return false;
        }
        self.properties.two_handed = true;
        if self.properties.inaccurate == true {
            self.properties.inaccurate = false;
        } else {
            self.properties.accurate = true;
        }
        self.weight += 2.0;
        self.value += 20;
        return true;
    }
    fn apply_recoil_compensating_stock_mod(&mut self) -> bool {
        if self.properties.two_handed == true {
            return false;
        }
        if self.properties.inaccurate == false {
            return false;
        }
        self.properties.two_handed = true;
        self.properties.inaccurate = false;
        self.fire_rate += 1;
        self.weight += 2.0;
        self.value += 3;
        return true;
    }

    ////////////////
    // SIGHT MODS //
    ////////////////

    fn apply_reflex_sight_mod(&mut self) -> bool {
        self.specialNotes
            .push_str("/n May re-roll hit location dice");
        self.value += 14;
        return true;
    }

    fn apply_short_scope_mod(&mut self) -> bool {
        if self.properties.accurate == true {
            return false;
        } else if self.properties.inaccurate == true {
            self.properties.inaccurate = false;
        } else {
            self.properties.accurate = true
        }
        self.weight += 1.0;
        self.value += 11;
        return true;
    }
    fn apply_long_scope_mod(&mut self) -> bool {
        match self.range {
            Range::Close => self.range = Range::Medium,
            Range::Medium => self.range = Range::Long,
            Range::Long => self.range = Range::Extream,
            Range::Extream => return false,
        }
        let tmp = self.apply_short_scope_mod();
        if tmp == false {
            return false;
        }
        self.value += 18;
        return true;
    }
    fn apply_short_night_scope(&mut self) -> bool {
        let tmp = self.apply_short_scope_mod();
        if tmp == false {
            return false;
        }
        if self.properties.nightvision == true {
            return false;
        }
        self.properties.nightvision = true;
        self.value += 27;
        return true;
    }
    fn apply_long_night_scope_mod(&mut self) -> bool {
        let tmp = self.apply_long_scope_mod();
        if tmp == false {
            return true;
        }
        if self.properties.nightvision == true {
            return false;
        }
        self.properties.nightvision = true;
        self.value += 21;
        return true;
    }
    fn apply_recon_scope_mod(&mut self) -> bool {
        if self.properties.accurate == true {
            return false;
        }
        if self.properties.recon == true {
            return false;
        }
        if self.properties.inaccurate == true {
            self.properties.inaccurate = false;
        } else {
            self.properties.accurate = true;
        }
        self.properties.recon = true;
        self.weight += 1.0;
        self.value += 59;
        return true;
    }

    /////////////////
    // MUZZLE MODS //
    /////////////////

    fn apply_bayonet_mod(&mut self) -> bool {
        self.specialNotes
            .push_str("/n Melee weapon, deals DR4, Peircing: 1, Physical damage type");
        self.weight += 2.0;
        self.value += 10;
        return true;
    }
    fn apply_compensator_mod(&mut self) -> bool {
        if self.properties.inaccurate == false {
            return true;
        }
        self.properties.inaccurate = false;
        self.weight += 1.0;
        self.value += 15;
        return true;
    }
    fn apply_muzzle_break_mod(&mut self) -> bool {
        if self.properties.inaccurate == false {
            return false;
        }
        self.properties.inaccurate = false;
        self.fire_rate += 1;
        self.weight += 1.0;
        self.value += 30;
        return true;
    }
}
//pub fn weapon_table_setup() {
//   let big_guns = vec![
//        "Fat Man",
//        "Flamer",
//        "Gatling Laser",
//        "Heavy Incinerator",
//        "Junk Jet",
//        "Minigun",
//        "Missile Launcher",
//    ];
//
//    let melee_weapons = vec![
//        "Sword",
//        "Combat Knife",
//        "Machete",
//        "Ripper",
//        "Shishkebab",
//        "Switchblade",
//        "Baseball bat",
//        "Aluminium Bat",
//        "Board",
//        "Lead Pipe",
//        "Pipe Wrench",
//       "Pool Que",
//        "Rolling Pin",
//        "Baton",
//        "Sledgehammer",
//        "Super Sledge",
//        "Tire Iron",
//        "walking Cane",
//        "Boxing Glove",
//        "Deathclaw Gauntlet",
//        "Brass Knuckles",
//        "Power Fist",
//    ];
//
//    let mut weapon_rarity = 10;
//    println!("do you want a modified weapon? Y/N");
//    let modded_weapon = crate::utility::convert_input_to_bool();
//    println!("do you want to specify a rarity? Y/N");
//    let set_rarity = crate::utility::convert_input_to_bool();
//    if set_rarity {
//        print!("enter the maximum rarity level:");
//        weapon_rarity = crate::utility::convert_input_to_int();
//    }
//    basic_weapon_table(modded_weapon, weapon_rarity);
//}
//pub fn basic_weapon_table(create_modded_weapon: bool, create_weapon_of_rarity_level: i32) {
//    println!("BASIC WEAPON TABLE GOES HERE");
//    println!(
//        "select weapon rarity range: {}",
//        create_weapon_of_rarity_level.to_string()
//    );
//    println!("modify weapon: {}", create_modded_weapon.to_string());
//if (create_modded_weapon) {
//    weapon_mod_table(incoming);
//}
//}
//pub fn random_weapon_of_rarity(incoming: i32) -> String {
//    let mut return_val = String::from("a weapon of rarity value:").to_owned();
//    return_val.push_str(&incoming.to_string());
//    return return_val;
//}
//pub fn weapon_mod_table(incoming: &String) {
//    println!("weapon mod goes here, after taking in the weapon type of {incoming}, and limmiting mods to availiable mods for said weapon");
//}

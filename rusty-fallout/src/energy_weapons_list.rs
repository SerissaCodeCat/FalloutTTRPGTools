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
            specialNotes: String::from(""),
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
            specialNotes: String::from(""),
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
            specialNotes: String::from(""),
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
            specialNotes: String::from(""),
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
            specialNotes: String::from(""),
        },
    ];


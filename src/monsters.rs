// ============================================================
//  monsters.rs — Monster definitions for each floor
// ============================================================

use rand::Rng;
use crate::types::*;

pub fn make_dire_rat(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "Dire Rat".into(),
        description: "A rat the size of a small dog, eyes glowing red in the darkness. \
                       It smells absolutely terrible.".into(),
        level: 1,
        hp: 12, max_hp: 12,
        attack_min: 2, attack_max: 5,
        defense: 0,
        exp_reward: 15, gold_reward: rng.gen_range(0..=3),
        loot: vec![
            (Item::consumable("Rat Meat", "Stringy, suspicious. Probably edible.",
                Rarity::Common, 1, 5), 40),
        ],
        is_boss: false,
        special_ability: None,
        special_cooldown: 0, special_current_cd: 0,
    }
}

pub fn make_hobgoblin(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "Hobgoblin Grunt".into(),
        description: "A grey-skinned hobgoblin wearing salvaged human clothing. \
                       It carries a jagged piece of rebar and looks hungry.".into(),
        level: 2,
        hp: 22, max_hp: 22,
        attack_min: 4, attack_max: 9,
        defense: 1,
        exp_reward: 30, gold_reward: rng.gen_range(2..=8),
        loot: vec![
            (Item::weapon("Rebar Club", "A heavy bent piece of rebar. Crude but painful.",
                Rarity::Common, 8, 4, 10), 25),
            (Item::consumable("Goblin Jerky", "Unidentified meat, heavily seasoned. Restores health.",
                Rarity::Common, 3, 8), 50),
        ],
        is_boss: false,
        special_ability: Some("Enrage: The hobgoblin screams and attacks twice.".into()),
        special_cooldown: 3, special_current_cd: 0,
    }
}

pub fn make_hobgoblin_shaman(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "Hobgoblin Shaman".into(),
        description: "Decorated in bones and wearing a traffic cone as a hat, \
                       this hobgoblin crackles with unstable dungeon magic.".into(),
        level: 3,
        hp: 18, max_hp: 18,
        attack_min: 6, attack_max: 12,
        defense: 0,
        exp_reward: 45, gold_reward: rng.gen_range(5..=15),
        loot: vec![
            (Item::mana_potion("Shaman's Vial",
                "A vial of stolen mana. Restores MP.", 20, 12), 60),
            (Item::weapon("Bone Wand", "A wand made from a human finger bone. Surprisingly effective.",
                Rarity::Uncommon, 30, 5, 11), 20),
        ],
        is_boss: false,
        special_ability: Some("Mana Bolt: Deals extra magic damage.".into()),
        special_cooldown: 2, special_current_cd: 0,
    }
}

pub fn make_zombie(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "Shambling Zombie".into(),
        description: "A recently deceased human still wearing office clothes. \
                       Its ID badge reads \"Dave from Accounting.\" It reaches for you.".into(),
        level: 1,
        hp: 20, max_hp: 20,
        attack_min: 3, attack_max: 6,
        defense: 1,
        exp_reward: 20, gold_reward: rng.gen_range(0..=5),
        loot: vec![
            (Item::consumable("Torn Wallet",
                "A dead guy's wallet. Has some cash still in it.", Rarity::Common, 10, 0), 70),
        ],
        is_boss: false,
        special_ability: Some("Infectious Bite: Has a chance to reduce max HP by 2.".into()),
        special_cooldown: 4, special_current_cd: 0,
    }
}

pub fn make_feral_dog(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "Feral Dog".into(),
        description: "Once someone's pet, now a snarling monster with too many teeth. \
                       It looks like it hasn't eaten in days.".into(),
        level: 1,
        hp: 14, max_hp: 14,
        attack_min: 3, attack_max: 8,
        defense: 0,
        exp_reward: 18, gold_reward: rng.gen_range(0..=2),
        loot: vec![
            (Item::consumable("Dog Tag",
                "A military-style dog tag. Might be worth something.", Rarity::Common, 5, 0), 30),
        ],
        is_boss: false,
        special_ability: Some("Pack Rush: Deals extra damage if another enemy is present.".into()),
        special_cooldown: 3, special_current_cd: 0,
    }
}

pub fn make_dungeon_imp(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "Dungeon Imp".into(),
        description: "A small, winged creature with a giant grin. It looks amused by your existence. \
                       The Borant Corporation placed these here for 'entertainment value.'".into(),
        level: 2,
        hp: 16, max_hp: 16,
        attack_min: 5, attack_max: 10,
        defense: 2,
        exp_reward: 35, gold_reward: rng.gen_range(5..=12),
        loot: vec![
            (Item::consumable("Imp Wing Powder",
                "Ground wing dust. Magically restores health.", Rarity::Uncommon, 15, 20), 35),
            (Item::throwable("Imp Grenade",
                "A tiny explosive the imp was carrying. Lethal.", Rarity::Uncommon, 20, 15, 30), 20),
        ],
        is_boss: false,
        special_ability: Some("Chaos Bolt: Random damage between 1 and 20.".into()),
        special_cooldown: 3, special_current_cd: 0,
    }
}

pub fn make_floor1_miniboss(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "Stumbles".into(),
        description: "A massive, bloated zombie in a police uniform. \
                       Someone has scrawled 'STUMBLES' on his forehead in lipstick. \
                       He is the size of a refrigerator and smells like one that's been off for a week.\n\
                       This is a named monster. The dungeon audience is watching closely.".into(),
        level: 3,
        hp: 60, max_hp: 60,
        attack_min: 8, attack_max: 15,
        defense: 2,
        exp_reward: 120, gold_reward: rng.gen_range(20..=40),
        loot: vec![
            (Item::armor("Police Vest",
                "A battered kevlar vest. Still provides some protection.",
                Rarity::Uncommon, 50, 3), 100),
            (Item::consumable("Medkit",
                "A proper first aid kit. Significantly restores health.",
                Rarity::Uncommon, 30, 40), 80),
        ],
        is_boss: false,
        special_ability: Some("Bile Vomit: Projectile vomit attack dealing 12-20 damage.".into()),
        special_cooldown: 3, special_current_cd: 0,
    }
}

pub fn make_floor1_boss(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "King Whiskers the Goblin Warlord".into(),
        description: "A four-foot-tall hobgoblin wearing a child's toy crown, \
                       a velvet bathrobe, and wielding a baseball bat wrapped in razor wire. \
                       He has an absolutely magnificent mustache. He is screaming something \
                       in Goblin that the system is translating as 'I AM THE MOST HANDSOME.' \
                       He is the Floor 1 Boss. The dungeon audience is going wild.".into(),
        level: 5,
        hp: 100, max_hp: 100,
        attack_min: 10, attack_max: 20,
        defense: 3,
        exp_reward: 250, gold_reward: rng.gen_range(50..=100),
        loot: vec![
            (Item::weapon("Razor Wire Bat",
                "King Whiskers' signature weapon. Wrapped in razor wire for extra cruelty.",
                Rarity::Rare, 100, 10, 22), 100),
            (Item::armor("Velvet Bathrobe of Majesty",
                "It's a bathrobe. It somehow makes you feel more charismatic.",
                Rarity::Rare, 80, 2), 100),
            (Item::consumable("Boss Chest Key",
                "Opens the treasure chest in the Staircase Room.",
                Rarity::Epic, 0, 0), 100),
        ],
        is_boss: true,
        special_ability: Some("Mustache Power: The warlord's magnificent mustache inspires him to attack twice.".into()),
        special_cooldown: 3, special_current_cd: 0,
    }
}

// ── Floor 2 Monsters ──────────────────────────────────────

pub fn make_cave_troll(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "Cave Troll".into(),
        description: "An enormous greenish humanoid with skin like tree bark. \
                       It regenerates slowly and smells of mushrooms and old cheese.".into(),
        level: 4,
        hp: 45, max_hp: 45,
        attack_min: 8, attack_max: 16,
        defense: 3,
        exp_reward: 70, gold_reward: rng.gen_range(5..=15),
        loot: vec![
            (Item::consumable("Troll Gland",
                "Pulsing with regenerative energy. Restores significant health.",
                Rarity::Uncommon, 25, 30), 50),
        ],
        is_boss: false,
        special_ability: Some("Regenerate: Heals 5 HP at the start of its turn.".into()),
        special_cooldown: 1, special_current_cd: 0,
    }
}

pub fn make_skeleton_archer(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "Skeleton Archer".into(),
        description: "A reanimated skeleton clutching a crossbow made of human bones. \
                       It clicks its jaw menacingly as you approach.".into(),
        level: 4,
        hp: 25, max_hp: 25,
        attack_min: 7, attack_max: 14,
        defense: 1,
        exp_reward: 55, gold_reward: rng.gen_range(3..=10),
        loot: vec![
            (Item::throwable("Bone Bolts (x3)",
                "Crossbow bolts made of sharpened bone. Actually quite effective.",
                Rarity::Common, 5, 10, 18), 60),
        ],
        is_boss: false,
        special_ability: Some("Aimed Shot: A precise shot that ignores some armor.".into()),
        special_cooldown: 3, special_current_cd: 0,
    }
}

pub fn make_dungeon_crawler_competitor(rng: &mut impl Rng) -> Monster {
    Monster {
        name: "Crawler: Braxis the Bold".into(),
        description: "Another dungeon crawler, a heavyset man in torn tactical gear. \
                       He eyes your cat hungrily. 'Nice cat,' he says. 'I'll take her \
                       after I kill you.' He has a knife.".into(),
        level: 4,
        hp: 40, max_hp: 40,
        attack_min: 7, attack_max: 13,
        defense: 2,
        exp_reward: 90, gold_reward: rng.gen_range(15..=35),
        loot: vec![
            (Item::consumable("Braxis's Medkit",
                "A well-stocked first aid kit. He won't need it anymore.",
                Rarity::Uncommon, 30, 40), 100),
            (Item::weapon("Combat Knife",
                "A proper military combat knife. Much better than a shiv.",
                Rarity::Uncommon, 45, 6, 13), 80),
        ],
        is_boss: false,
        special_ability: Some("Dirty Fighting: Temporarily reduces your attack by 2.".into()),
        special_cooldown: 4, special_current_cd: 0,
    }
}

pub fn make_floor2_boss(_rng: &mut impl Rng) -> Monster {
    Monster {
        name: "The Iron Duchess".into(),
        description: "She was a dungeon administrator before she... got promoted. \
                       Now she's eight feet tall, covered in black iron armor fused to her skin, \
                       and wields a flail made of dungeon tokens. Her laugh sounds like a \
                       fire alarm. She screams: 'YOU ARE RATED PG-13 CONTENT!'\n\
                       FLOOR 2 BOSS. This is a named monster. Kill her to reach Floor 3.".into(),
        level: 7,
        hp: 150, max_hp: 150,
        attack_min: 14, attack_max: 25,
        defense: 5,
        exp_reward: 400, gold_reward: 150,
        loot: vec![
            (Item::armor("Iron Duchess Pauldron",
                "One enormous shoulder plate ripped from her armor. Heavy but excellent protection.",
                Rarity::Epic, 200, 7), 100),
            (Item::consumable("Dungeon Token Stash",
                "A pile of dungeon tokens — converts to gold automatically.",
                Rarity::Rare, 200, 0), 100),
            (Item::weapon("Duchess's Flail",
                "A flail made of dungeon tokens. Heavy and fast.",
                Rarity::Epic, 180, 14, 26), 100),
        ],
        is_boss: true,
        special_ability: Some("Rating Warning: A burst of bureaucratic energy deals 18-28 damage to all.".into()),
        special_cooldown: 3, special_current_cd: 0,
    }
}

// ── Floor 3 Boss ──────────────────────────────────────────

pub fn make_floor3_boss(_rng: &mut impl Rng) -> Monster {
    Monster {
        name: "The Maestro of the Meat Grinder".into(),
        description: "A towering construct of flesh, metal, and dungeon magic. \
                       It looks like a giant blender designed by a sadist. \
                       Borant's finest engineering: an automated killing machine \
                       put here to remind crawlers that the dungeon is not a game.\n\
                       (It is, in fact, a game. But a deadly one.)\n\
                       FLOOR 3 BOSS. The final obstacle before the surface. Survive this \
                       and you reach Floor 4 — if you dare.".into(),
        level: 10,
        hp: 200, max_hp: 200,
        attack_min: 18, attack_max: 30,
        defense: 6,
        exp_reward: 600, gold_reward: 250,
        loot: vec![
            (Item::weapon("Grinder Arm",
                "Torn from the Maestro himself. Functions as a heavy bladed weapon.",
                Rarity::Legendary, 500, 18, 32), 100),
            (Item::consumable("Floor 3 Exit Key",
                "Congratulations. You survived Floor 3.", Rarity::Legendary, 0, 0), 100),
        ],
        is_boss: true,
        special_ability: Some("Grind Cycle: A devastating spinning attack hitting for 25-40 damage.".into()),
        special_cooldown: 3, special_current_cd: 0,
    }
}

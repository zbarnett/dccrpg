// ============================================================
//  items.rs — Item factory functions and loot box system
// ============================================================

use rand::Rng;
use crate::types::*;

// ── Loot Box Generation ───────────────────────────────────

pub struct LootBox {
    pub rarity: Rarity,
    pub items: Vec<Item>,
}

pub fn generate_loot_box(rarity: Rarity, floor: u32, rng: &mut impl Rng) -> LootBox {
    let item_count = match rarity {
        Rarity::Common    => 1,
        Rarity::Uncommon  => rng.gen_range(1..=2),
        Rarity::Rare      => rng.gen_range(2..=3),
        Rarity::Epic      => rng.gen_range(2..=4),
        Rarity::Legendary => rng.gen_range(3..=5),
    };

    let mut items = Vec::new();
    for _ in 0..item_count {
        items.push(random_item_for_floor(&rarity, floor, rng));
    }

    LootBox { rarity, items }
}

fn random_item_for_floor(rarity: &Rarity, floor: u32, rng: &mut impl Rng) -> Item {
    match rarity {
        Rarity::Common => {
            let r = rng.gen_range(0..8);
            match r {
                0 => Item::consumable("Protein Bar",
                    "Stale but edible. Restores some health.", Rarity::Common, 5, 15),
                1 => Item::consumable("Flask of Water",
                    "Clean water. Restores a small amount of health.", Rarity::Common, 3, 10),
                2 => Item::consumable("Bandage Roll",
                    "A roll of gauze. Restores health.", Rarity::Common, 8, 20),
                3 => Item::consumable("Energy Drink",
                    "Tastes like battery acid. Restores health.", Rarity::Common, 5, 12),
                4 => Item::consumable("Cigarettes",
                    "A pack of cigarettes. Calming. Restores a small amount of health.",
                    Rarity::Common, 3, 5),
                5 => Item::consumable("Canned Beans",
                    "Cold beans straight from the can. Restores health.", Rarity::Common, 4, 18),
                6 => Item::armor("Scrap Metal Vest",
                    "Pieces of metal duct-taped together. Surprisingly effective.",
                    Rarity::Common, 10, 1),
                _ => Item::consumable("Rat Jerky",
                    "Dried rat meat. Not as bad as it sounds.", Rarity::Common, 2, 8),
            }
        }
        Rarity::Uncommon => {
            let r = rng.gen_range(0..8);
            match r {
                0 => Item::consumable("Medkit",
                    "A proper first aid kit. Significantly restores health.",
                    Rarity::Uncommon, 30, 40),
                1 => Item::consumable("Goblin Jerky",
                    "Heavily seasoned mystery meat. Restores good health.",
                    Rarity::Uncommon, 10, 25),
                2 => Item::weapon("Kitchen Knife",
                    "A proper chef's knife. Balanced and sharp.",
                    Rarity::Uncommon, 25, 5, 11),
                3 => Item::weapon("Fire Poker",
                    "A heavy iron poker from a fireplace set.",
                    Rarity::Uncommon, 20, 5, 12),
                4 => Item::armor("Leather Jacket",
                    "A biker's leather jacket. Provides decent protection.",
                    Rarity::Uncommon, 35, 2),
                5 => Item::armor("Hard Hat",
                    "A construction worker's hard hat. Protects your head.",
                    Rarity::Uncommon, 15, 2),
                6 => Item::mana_potion("Mana Potion",
                    "A vial of glowing blue liquid. Restores MP.", 15, 15),
                _ => Item::throwable("Molotov Cocktail",
                    "A glass bottle filled with flaming alcohol. Handle with care.",
                    Rarity::Uncommon, 20, 18, 30),
            }
        }
        Rarity::Rare => {
            let r = rng.gen_range(0..6);
            match r {
                0 => Item::weapon(
                    if floor >= 2 { "Combat Sword" } else { "Fire Axe" },
                    "A proper combat-grade bladed weapon.",
                    Rarity::Rare, 80, 9, 17),
                1 => Item::armor("Riot Gear Vest",
                    "Partial riot police armor. Excellent protection.",
                    Rarity::Rare, 75, 5),
                2 => Item::consumable("Super Medkit",
                    "Military-grade medical supplies. Massively restores health.",
                    Rarity::Rare, 60, 60),
                3 => Item::mana_potion("Greater Mana Potion",
                    "A large vial of concentrated mana. Restores a lot of MP.", 40, 30),
                4 => Item::throwable("Grenade",
                    "A military-grade fragmentation grenade.",
                    Rarity::Rare, 50, 30, 55),
                _ => Item::consumable("Adrenaline Shot",
                    "Military-grade stimulant. Massively restores health.",
                    Rarity::Rare, 50, 50),
            }
        }
        Rarity::Epic => {
            let r = rng.gen_range(0..4);
            match r {
                0 => Item::weapon("Enchanted Blade",
                    "A blade that hums with dungeon magic. Exceptionally deadly.",
                    Rarity::Epic, 200, 14, 24),
                1 => Item::armor("Crawler's Vest",
                    "Purpose-built dungeon armor. The Borant logo is on the back.",
                    Rarity::Epic, 180, 7),
                2 => Item::consumable("Resurrection Vial",
                    "A single-use vial. Fully restores HP and MP. Rare beyond words.",
                    Rarity::Epic, 300, 100),
                _ => Item::throwable("Arcane Bomb",
                    "A bomb infused with dungeon magic. Catastrophically damaging.",
                    Rarity::Epic, 150, 45, 80),
            }
        }
        Rarity::Legendary => {
            let r = rng.gen_range(0..3);
            match r {
                0 => Item::weapon("Donut's Tiara of Doom",
                    "Princess Donut insists this is hers. It radiates power. \
                     You'll have to pry it from her... actually, she gave it to you.",
                    Rarity::Legendary, 1000, 20, 35),
                1 => Item::armor("Sponsor Exo-Suit Fragment",
                    "Part of a powered exo-suit sent by a generous sponsor. \
                     Provides exceptional protection.",
                    Rarity::Legendary, 800, 10),
                _ => Item::consumable("The Vial of Champions",
                    "A Legendary consumable. Fully restores HP, MP, and increases max HP by 20 permanently.",
                    Rarity::Legendary, 500, 150),
            }
        }
    }
}

/// Roll a random loot box rarity (weighted)
pub fn random_box_rarity(rng: &mut impl Rng) -> Rarity {
    let roll = rng.gen_range(0..100);
    match roll {
        0..=49  => Rarity::Common,
        50..=74 => Rarity::Uncommon,
        75..=89 => Rarity::Rare,
        90..=96 => Rarity::Epic,
        _       => Rarity::Legendary,
    }
}

pub fn sponsor_box_rarity(rng: &mut impl Rng) -> Rarity {
    let roll = rng.gen_range(0..100);
    match roll {
        0..=29  => Rarity::Uncommon,
        30..=59 => Rarity::Rare,
        60..=84 => Rarity::Epic,
        _       => Rarity::Legendary,
    }
}

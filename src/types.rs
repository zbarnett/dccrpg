// ============================================================
//  types.rs — Core data structures and enums
// ============================================================

use rand::Rng;

// ANSI color constants
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const BOLD: &str = "\x1b[1m";
pub const RESET: &str = "\x1b[0m";
pub const DIM: &str = "\x1b[2m";

// ── Classes ────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Class {
    Unclassed,
    Punk,
    Hooligan,
    Skirmisher,
    Sapper,
    ApprenticeWizard,
}

impl Class {
    pub fn name(&self) -> &str {
        match self {
            Class::Unclassed        => "Unclassed",
            Class::Punk             => "Punk",
            Class::Hooligan         => "Hooligan",
            Class::Skirmisher       => "Skirmisher",
            Class::Sapper           => "Sapper",
            Class::ApprenticeWizard => "Apprentice Wizard",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Class::Unclassed =>
                "No class selected.",
            Class::Punk =>
                "Street-tough brawler. High Strength and Constitution.\n  \
                 Bonus damage with crude weapons. Starts with Brass Knuckles.",
            Class::Hooligan =>
                "Quick and dirty fighter. High Dexterity and Charisma.\n  \
                 Can attempt to pickpocket. Starts with a Shiv.",
            Class::Skirmisher =>
                "Hit-and-run specialist. High Dexterity.\n  \
                 Bonus to ranged attacks. Starts with a Sling and Rocks.",
            Class::Sapper =>
                "Demolitions expert. High Intelligence.\n  \
                 Bonus damage with explosives. Starts with 3 Pipe Bombs and a Wrench.",
            Class::ApprenticeWizard =>
                "Barely trained magic user. High Intelligence and Wisdom.\n  \
                 Can cast spells. Starts with a Chipped Wand and 25 MP.",
        }
    }

    /// Returns (str, dex, con, int, wis, cha) bonuses
    pub fn stat_bonuses(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Class::Unclassed        => (0, 0, 0, 0, 0, 0),
            Class::Punk             => (3, 0, 3, 0, 0, 0),
            Class::Hooligan         => (0, 3, 1, 0, 0, 2),
            Class::Skirmisher       => (0, 4, 0, 1, 0, 0),
            Class::Sapper           => (0, 2, 0, 3, 0, 0),
            Class::ApprenticeWizard => (0, 0, 0, 4, 2, 0),
        }
    }
}

// ── Item Rarity ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub fn color(&self) -> &'static str {
        match self {
            Rarity::Common    => WHITE,
            Rarity::Uncommon  => GREEN,
            Rarity::Rare      => BLUE,
            Rarity::Epic      => MAGENTA,
            Rarity::Legendary => YELLOW,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Rarity::Common    => "Common",
            Rarity::Uncommon  => "Uncommon",
            Rarity::Rare      => "Rare",
            Rarity::Epic      => "Epic",
            Rarity::Legendary => "Legendary",
        }
    }
}

// ── Item ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum ItemKind {
    Weapon,
    Armor,
    Consumable,
    Throwable,
    QuestItem,
    ManaPotion,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub kind: ItemKind,
    pub rarity: Rarity,
    pub value: u32,
    pub damage_min: u32,
    pub damage_max: u32,
    pub armor_value: u32,
    pub heal_amount: u32,
    pub mana_amount: u32,
    pub throw_dmg_min: u32,
    pub throw_dmg_max: u32,
    pub quantity: u32,
}

impl Item {
    pub fn weapon(name: &str, desc: &str, rarity: Rarity, value: u32, dmin: u32, dmax: u32) -> Self {
        Item { name: name.into(), description: desc.into(), kind: ItemKind::Weapon,
               rarity, value, damage_min: dmin, damage_max: dmax,
               armor_value: 0, heal_amount: 0, mana_amount: 0,
               throw_dmg_min: 0, throw_dmg_max: 0, quantity: 1 }
    }
    pub fn armor(name: &str, desc: &str, rarity: Rarity, value: u32, av: u32) -> Self {
        Item { name: name.into(), description: desc.into(), kind: ItemKind::Armor,
               rarity, value, damage_min: 0, damage_max: 0,
               armor_value: av, heal_amount: 0, mana_amount: 0,
               throw_dmg_min: 0, throw_dmg_max: 0, quantity: 1 }
    }
    pub fn consumable(name: &str, desc: &str, rarity: Rarity, value: u32, heal: u32) -> Self {
        Item { name: name.into(), description: desc.into(), kind: ItemKind::Consumable,
               rarity, value, damage_min: 0, damage_max: 0,
               armor_value: 0, heal_amount: heal, mana_amount: 0,
               throw_dmg_min: 0, throw_dmg_max: 0, quantity: 1 }
    }
    pub fn mana_potion(name: &str, desc: &str, value: u32, mana: u32) -> Self {
        Item { name: name.into(), description: desc.into(), kind: ItemKind::ManaPotion,
               rarity: Rarity::Uncommon, value, damage_min: 0, damage_max: 0,
               armor_value: 0, heal_amount: 0, mana_amount: mana,
               throw_dmg_min: 0, throw_dmg_max: 0, quantity: 1 }
    }
    pub fn throwable(name: &str, desc: &str, rarity: Rarity, value: u32, dmin: u32, dmax: u32) -> Self {
        Item { name: name.into(), description: desc.into(), kind: ItemKind::Throwable,
               rarity, value, damage_min: 0, damage_max: 0,
               armor_value: 0, heal_amount: 0, mana_amount: 0,
               throw_dmg_min: dmin, throw_dmg_max: dmax, quantity: 1 }
    }
    pub fn display_name(&self) -> String {
        format!("{}{}{}{}",
            self.rarity.color(), BOLD, self.name, RESET)
    }
}

// ── Companion (Princess Donut) ─────────────────────────────

#[derive(Debug, Clone)]
pub struct Companion {
    pub name: String,
    pub class: String,
    pub level: u32,
    pub hp: i32,
    pub max_hp: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub alive: bool,
    pub exp: u64,
    pub exp_to_next: u64,
}

impl Companion {
    pub fn new() -> Self {
        Companion {
            name: "Princess Donut".into(),
            class: "Noble Cat".into(),
            level: 1,
            hp: 20,
            max_hp: 20,
            attack_min: 3,
            attack_max: 8,
            alive: true,
            exp: 0,
            exp_to_next: 80,
        }
    }

    pub fn attack(&self, rng: &mut impl Rng) -> i32 {
        rng.gen_range(self.attack_min..=self.attack_max)
    }

    pub fn gain_exp(&mut self, amount: u64) -> bool {
        self.exp += amount;
        if self.exp >= self.exp_to_next {
            self.level += 1;
            self.exp -= self.exp_to_next;
            self.exp_to_next = (self.exp_to_next as f64 * 1.6) as u64;
            self.max_hp += 5;
            self.hp = self.max_hp;
            self.attack_min += 1;
            self.attack_max += 2;
            true
        } else {
            false
        }
    }
}

// ── Monster ───────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Monster {
    pub name: String,
    pub description: String,
    pub level: u32,
    pub hp: i32,
    pub max_hp: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
    pub exp_reward: u64,
    pub gold_reward: u32,
    pub loot: Vec<(Item, u32)>,  // (item, drop_chance_0_to_100)
    pub is_boss: bool,
    pub special_ability: Option<String>,
    pub special_cooldown: u32,
    pub special_current_cd: u32,
}

impl Monster {
    pub fn is_alive(&self) -> bool { self.hp > 0 }
}

// ── Player ────────────────────────────────────────────────

pub struct Player {
    pub name: String,
    pub level: u32,
    pub exp: u64,
    pub exp_to_next: u64,
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub class: Class,
    pub inventory: Vec<Item>,
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub gold: u32,
    pub floor: u32,
    pub viewers: u64,
    pub kills: u32,
    pub rooms_cleared: u32,
    pub boxes_opened: u32,
    pub donut: Companion,
    pub achievements: Vec<String>,
    pub pipe_bombs: u32,
    pub escaped_fights: u32,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.into(),
            level: 1, exp: 0, exp_to_next: 100,
            hp: 30, max_hp: 30,
            mp: 0, max_mp: 0,
            strength: 5, dexterity: 5, constitution: 5,
            intelligence: 5, wisdom: 5, charisma: 5,
            class: Class::Unclassed,
            inventory: Vec::new(),
            weapon: None, armor: None,
            gold: 10, floor: 1,
            viewers: 1_200_000,
            kills: 0, rooms_cleared: 0, boxes_opened: 0,
            donut: Companion::new(),
            achievements: Vec::new(),
            pipe_bombs: 0,
            escaped_fights: 0,
        }
    }

    pub fn apply_class(&mut self, class: Class) {
        let (sb, db, cb, ib, wb, chb) = class.stat_bonuses();
        self.strength     += sb;
        self.dexterity    += db;
        self.constitution += cb;
        self.intelligence += ib;
        self.wisdom       += wb;
        self.charisma     += chb;

        self.max_hp = 30 + (self.constitution - 5) * 5;
        self.hp = self.max_hp;

        if matches!(class, Class::ApprenticeWizard) {
            self.max_mp = 25 + (self.intelligence - 5) * 3 + (self.wisdom - 5) * 2;
        } else {
            self.max_mp = (self.intelligence - 5).max(0) * 2;
        }
        self.mp = self.max_mp;

        match &class {
            Class::Punk => {
                self.weapon = Some(Item::weapon(
                    "Brass Knuckles",
                    "Cold brass wrapped around your knuckles. Street-proven.",
                    Rarity::Common, 15, 3, 8));
                self.inventory.push(Item::consumable(
                    "Protein Bar", "Stale but edible. Restores health.",
                    Rarity::Common, 5, 15));
                self.inventory.push(Item::consumable(
                    "Protein Bar", "Stale but edible. Restores health.",
                    Rarity::Common, 5, 15));
            }
            Class::Hooligan => {
                self.weapon = Some(Item::weapon(
                    "Shiv",
                    "A sharpened piece of metal wrapped in electrical tape.",
                    Rarity::Common, 10, 4, 10));
                self.inventory.push(Item::consumable(
                    "Flask of Whiskey", "Burns going down. Restores health.",
                    Rarity::Common, 8, 12));
                self.inventory.push(Item::consumable(
                    "Cigarettes", "Surprisingly calming. Restores a small amount of health.",
                    Rarity::Common, 3, 5));
            }
            Class::Skirmisher => {
                self.weapon = Some(Item::weapon(
                    "Sling",
                    "A simple leather sling. Requires rocks to use.",
                    Rarity::Common, 8, 3, 9));
                self.inventory.push(Item::throwable(
                    "Smooth Rocks (x5)", "A handful of sling-worthy rocks.",
                    Rarity::Common, 2, 5, 12));
                self.inventory.push(Item::consumable(
                    "Energy Drink", "Tastes like battery acid. Restores health.",
                    Rarity::Common, 5, 12));
            }
            Class::Sapper => {
                self.pipe_bombs = 3;
                self.weapon = Some(Item::weapon(
                    "Heavy Wrench",
                    "A solid monkey wrench. Practical in more ways than one.",
                    Rarity::Common, 12, 4, 9));
                self.inventory.push(Item::throwable(
                    "Pipe Bomb", "Homemade explosive. Handle with extreme care.",
                    Rarity::Uncommon, 25, 25, 50));
                self.inventory.push(Item::throwable(
                    "Pipe Bomb", "Homemade explosive. Handle with extreme care.",
                    Rarity::Uncommon, 25, 25, 50));
                self.inventory.push(Item::throwable(
                    "Pipe Bomb", "Homemade explosive. Handle with extreme care.",
                    Rarity::Uncommon, 25, 25, 50));
            }
            Class::ApprenticeWizard => {
                self.weapon = Some(Item::weapon(
                    "Chipped Wand",
                    "A cracked wand salvaged from a wizard's dumpster. Still works, mostly.",
                    Rarity::Common, 15, 2, 6));
                self.inventory.push(Item::mana_potion(
                    "Mana Potion",
                    "A small vial of glowing blue liquid. Restores MP.",
                    15, 15));
                self.inventory.push(Item::mana_potion(
                    "Mana Potion",
                    "A small vial of glowing blue liquid. Restores MP.",
                    15, 15));
            }
            _ => {}
        }

        self.class = class;
    }

    pub fn melee_damage(&self, rng: &mut impl Rng) -> i32 {
        let base = if let Some(w) = &self.weapon {
            rng.gen_range(w.damage_min..=w.damage_max) as i32
        } else {
            rng.gen_range(1_u32..=4) as i32
        };
        let str_bonus = (self.strength - 5) / 2;
        (base + str_bonus).max(1)
    }

    pub fn spell_damage(&self, rng: &mut impl Rng) -> i32 {
        let base = rng.gen_range(8_u32..=18) as i32;
        let int_bonus = (self.intelligence - 5) / 2;
        (base + int_bonus).max(5)
    }

    pub fn defense_value(&self) -> i32 {
        let base = self.armor.as_ref().map(|a| a.armor_value as i32).unwrap_or(0);
        base + (self.dexterity - 5) / 3
    }

    /// Returns true if levelled up
    pub fn gain_exp(&mut self, amount: u64) -> bool {
        self.exp += amount;
        if self.exp >= self.exp_to_next {
            self.exp -= self.exp_to_next;
            self.exp_to_next = (self.exp_to_next as f64 * 1.5) as u64;
            self.level += 1;
            self.strength     += 1;
            self.dexterity    += 1;
            self.constitution += 1;
            self.intelligence += 1;
            self.wisdom       += 1;
            let hp_gain = 5 + (self.constitution - 5) / 2;
            let mp_gain = if self.max_mp > 0 { 3 } else { 0 };
            self.max_hp += hp_gain;
            self.max_mp += mp_gain;
            self.hp = self.max_hp;
            self.mp = self.max_mp;
            true
        } else {
            false
        }
    }

    pub fn add_viewers(&mut self, amount: u64) {
        self.viewers = self.viewers.saturating_add(amount);
    }

    pub fn unlock_achievement(&mut self, name: &str) -> bool {
        if !self.achievements.contains(&name.to_string()) {
            self.achievements.push(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn take_item_from_inv(&mut self, idx: usize) -> Option<Item> {
        if idx < self.inventory.len() {
            Some(self.inventory.remove(idx))
        } else {
            None
        }
    }
}

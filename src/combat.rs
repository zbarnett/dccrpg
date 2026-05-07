// ============================================================
//  combat.rs — Turn-based combat system
// ============================================================

use rand::Rng;
use crate::types::*;
use crate::display::*;

pub enum CombatResult {
    Victory,
    Defeat,
    Fled,
}

fn donut_attack_flavor(rng: &mut impl Rng) -> &'static str {
    const FLAVORS: &[&str] = &[
        "launches herself face-first",
        "unleashes a fury of claws",
        "bites with surprising viciousness",
        "performs a beautifully executed pounce",
        "yowls a judgmental battle cry and strikes",
        "bats at the target with royal contempt",
        "delivers a precise, elegant swipe",
        "lands a claws-first dropkick",
    ];
    FLAVORS[rng.gen_range(0..FLAVORS.len())]
}

pub fn run_combat(player: &mut Player, mut monster: Monster, rng: &mut impl Rng) -> CombatResult {
    println!();
    println!("{}⚔  ENCOUNTER: {}{}{}", YELLOW, RED, monster.name, RESET);
    println!("{}{}{}", DIM, monster.description, RESET);
    press_enter();

    loop {
        print_combat_hud(player, &monster);

        // Build combat menu
        let mut options: Vec<String> = vec![
            format!("Attack{}", if let Some(w) = &player.weapon {
                format!(" ({})", w.name)
            } else {
                " (Unarmed)".to_string()
            }),
            "Use Item".to_string(),
            format!("Send Donut{}", if player.donut.alive { "" } else { " [INCAPACITATED]" }),
        ];

        if matches!(player.class, Class::ApprenticeWizard) && player.mp >= 8 {
            options.push(format!("Cast Spell (8 MP) — {}mp remaining{}", player.mp, RESET));
        }
        if player.class == Class::Sapper && player.pipe_bombs > 0 {
            options.push(format!("Throw Pipe Bomb ({} remaining)", player.pipe_bombs));
        }
        options.push("Flee (50% chance)".to_string());

        let opt_refs: Vec<&str> = options.iter().map(|s| s.as_str()).collect();
        let choice = get_choice("--- Your Move ---", &opt_refs);

        match choice {
            0 => {
                // Attack
                let dmg = player.melee_damage(rng);
                let final_dmg = (dmg - monster.defense).max(1);
                monster.hp -= final_dmg;
                println!("  {}You attack {} for {}{}{} damage!{}",
                    WHITE, monster.name, RED, final_dmg, WHITE, RESET);
            }
            1 => {
                // Use Item
                if player.inventory.is_empty() {
                    println!("  {}Your inventory is empty.{}", RED, RESET);
                    continue; // Don't advance monster turn
                }
                let used = use_item_menu(player, rng);
                if !used {
                    continue;
                }
            }
            2 => {
                // Donut attack
                if !player.donut.alive {
                    println!("  {}Princess Donut is incapacitated!{}", RED, RESET);
                    continue;
                }
                let dmg = player.donut.attack(rng);
                let final_dmg = (dmg - monster.defense).max(1);
                monster.hp -= final_dmg;
                let flavor = donut_attack_flavor(rng);
                println!("  {}🐱 Princess Donut {} for {}{}{} damage!{}",
                    MAGENTA, flavor, RED, final_dmg, MAGENTA, RESET);
            }
            3 if matches!(player.class, Class::ApprenticeWizard) && player.mp >= 8 => {
                // Spell
                let dmg = player.spell_damage(rng);
                player.mp -= 8;
                println!("  {}✨ You cast a magic bolt at {} for {}{}{} magic damage!{}",
                    BLUE, monster.name, RED, dmg, BLUE, RESET);
                monster.hp -= dmg;
            }
            idx if player.class == Class::Sapper && player.pipe_bombs > 0
                   && idx == options.len() - 2 => {
                // Pipe bomb
                let dmg = rng.gen_range(20_i32..=45);
                let splash = rng.gen_range(5_i32..=15);
                player.pipe_bombs -= 1;
                player.hp -= splash; // Self-damage from explosion
                monster.hp -= dmg;
                println!("  {}💥 BOOM! Pipe bomb explodes for {}{}{} damage!{}", YELLOW, RED, dmg, YELLOW, RESET);
                println!("  {}The explosion also catches you for {}{}{} splash damage!{}", YELLOW, RED, splash, YELLOW, RESET);
            }
            _ => {
                // Flee
                let flee_chance = 50 + (player.dexterity - 5) * 5;
                let roll: i32 = rng.gen_range(0..100);
                if roll < flee_chance {
                    println!("  {}You sprint away as fast as you can. Princess Donut follows, \
                               but stops to hiss at the monster first.{}", DIM, RESET);
                    player.escaped_fights += 1;
                    return CombatResult::Fled;
                } else {
                    println!("  {}You try to flee but {} blocks your escape!{}", RED, monster.name, RESET);
                }
            }
        }

        // Check if monster dead
        if !monster.is_alive() {
            return victory(player, &monster, rng);
        }

        // Monster turn
        monster_turn(player, &mut monster, rng);

        if player.hp <= 0 {
            println!();
            println!("  {}You have been slain by {}...{}", RED, monster.name, RESET);
            return CombatResult::Defeat;
        }
    }
}

fn monster_turn(player: &mut Player, monster: &mut Monster, rng: &mut impl Rng) {
    // Maybe use special ability
    if let Some(ref ability) = monster.special_ability.clone() {
        if monster.special_current_cd == 0 && rng.gen_range(0..100) < 30 {
            monster.special_current_cd = monster.special_cooldown;
            println!();
            println!("  {}⚠ {} uses their special: {}{}{}!{}", RED, monster.name, YELLOW, ability, RED, RESET);

            // Special effects
            let dmg = if monster.name.contains("Stumbles") {
                rng.gen_range(12_i32..=20)
            } else if monster.name.contains("King Whiskers") {
                // Attacks twice
                let d1 = (rng.gen_range(monster.attack_min..=monster.attack_max) - player.defense_value()).max(1);
                let d2 = (rng.gen_range(monster.attack_min..=monster.attack_max) - player.defense_value()).max(1);
                player.hp -= d1;
                println!("  {}{}  {} ATTACKS TWICE! First hit: {}{} damage!{}{}",
                    RED, BOLD, monster.name, d1, RED, RESET, RESET);
                d2
            } else if monster.name.contains("Maestro") {
                rng.gen_range(25_i32..=40)
            } else if monster.name.contains("Iron Duchess") {
                rng.gen_range(18_i32..=28)
            } else if monster.name.contains("Imp") {
                rng.gen_range(1_i32..=20)
            } else {
                rng.gen_range(monster.attack_min..=monster.attack_max)
            };

            let final_dmg = (dmg - player.defense_value()).max(1);

            // Zombie infectious bite: chance to reduce max HP
            if monster.name.contains("Zombie") && rng.gen_range(0..100) < 30 {
                player.max_hp = (player.max_hp - 2).max(1);
                if player.hp > player.max_hp { player.hp = player.max_hp; }
                println!("  {}The zombie's bite feels infected! Your max HP is reduced by 2.{}", RED, RESET);
            }

            // Troll regenerate
            if monster.name.contains("Troll") {
                let regen = 5;
                monster.hp = (monster.hp + regen).min(monster.max_hp);
                println!("  {}The troll regenerates {} HP!{}", GREEN, regen, RESET);
            }

            player.hp -= final_dmg;
            println!("  {}{}  {} hits you for {}{} damage!{}", RED, BOLD, monster.name, final_dmg, RED, RESET);

            // Donut might take splash
            if player.donut.alive && rng.gen_range(0..100) < 20 {
                let donut_dmg = rng.gen_range(1..=4);
                player.donut.hp -= donut_dmg;
                println!("  {}🐱 Princess Donut takes {} splash damage!{}", MAGENTA, donut_dmg, RESET);
                if player.donut.hp <= 0 {
                    player.donut.alive = false;
                    player.donut.hp = 0;
                    println!("  {}🐱 Princess Donut has been incapacitated! She hisses furiously.{}", RED, RESET);
                }
            }

            return;
        }
    }

    // Cooldown reduction
    if monster.special_current_cd > 0 {
        monster.special_current_cd -= 1;
    }

    // Troll regeneration (even on normal turns)
    if monster.name.contains("Troll") {
        let regen = 3;
        monster.hp = (monster.hp + regen).min(monster.max_hp);
        println!("  {}The troll's wounds slowly close (+{} HP).{}", GREEN, regen, RESET);
    }

    // Normal attack
    let raw = rng.gen_range(monster.attack_min..=monster.attack_max);

    // Check if Donut intercepts (10% chance if alive)
    if player.donut.alive && rng.gen_range(0..100) < 10 {
        let donut_def = 1;
        let dmg = (raw - donut_def).max(1);
        player.donut.hp -= dmg;
        println!("  {}🐱 Princess Donut leaps in front of the attack! She takes {}{}{} damage!{}",
            MAGENTA, RED, dmg, MAGENTA, RESET);
        if player.donut.hp <= 0 {
            player.donut.alive = false;
            player.donut.hp = 0;
            println!("  {}🐱 Princess Donut has been incapacitated! She collapses with a furious meow.{}", RED, RESET);
        }
    } else {
        let final_dmg = (raw - player.defense_value()).max(1);
        player.hp -= final_dmg;
        println!("  {}{}  {} attacks you for {} damage!{}{}", RED, BOLD, monster.name, final_dmg, RESET, RESET);
    }
}

fn victory(player: &mut Player, monster: &Monster, rng: &mut impl Rng) -> CombatResult {
    println!();
    println!("  {}☠  {} has been defeated!{}", GREEN, monster.name, RESET);
    println!("  {}+{} EXP  +{} gold{}", YELLOW, monster.exp_reward, monster.gold_reward, RESET);

    player.gold += monster.gold_reward;
    player.kills += 1;

    let levelled = player.gain_exp(monster.exp_reward);
    let donut_levelled = player.donut.gain_exp(monster.exp_reward / 2);

    // Restore Donut if incapacitated (they're resilient)
    if !player.donut.alive {
        player.donut.alive = true;
        player.donut.hp = player.donut.max_hp / 2;
        println!("  {}🐱 Princess Donut shakes herself off and stands back up. She glares at you.{}", MAGENTA, RESET);
    }

    if levelled {
        println!();
        println!("{}╔══════════════════════════════════════╗{}", CYAN, RESET);
        println!("{}║   ⬆  LEVEL UP! You are now Level {}  ║{}", CYAN, player.level, RESET);
        println!("{}║   All stats increased. Full HP/MP.   ║{}", CYAN, RESET);
        println!("{}╚══════════════════════════════════════╝{}", CYAN, RESET);
    }

    if donut_levelled {
        println!("  {}🐱 Princess Donut levelled up to Lv.{}! She preens triumphantly.{}", MAGENTA, player.donut.level, RESET);
    }

    player.add_viewers(monster.exp_reward as u64 * 500);

    // Loot drops
    for (item, chance) in &monster.loot {
        if rng.gen_range(0..100) < *chance {
            println!("  {}💰 {} dropped: {}{}{}{}",
                YELLOW, monster.name, item.rarity.color(), BOLD, item.name, RESET);
            player.inventory.push(item.clone());
        }
    }

    // Boss reward: a crawler box
    if monster.is_boss {
        let rarity = Rarity::Rare;
        let loot_box = crate::items::generate_loot_box(rarity.clone(), player.floor, rng);
        println!();
        println!("{}★  BOSS DEFEATED — CRAWLER BOX AWARDED!  ★{}", YELLOW, RESET);
        let item_refs: Vec<&Item> = loot_box.items.iter().collect();
        print_loot_box(&rarity, &item_refs);
        for item in loot_box.items {
            player.inventory.push(item);
        }
        player.boxes_opened += 1;
    }

    CombatResult::Victory
}

fn use_item_menu(player: &mut Player, rng: &mut impl Rng) -> bool {
    if player.inventory.is_empty() {
        println!("  {}Your inventory is empty.{}", RED, RESET);
        return false;
    }

    println!("\n  {}--- INVENTORY ---{}", CYAN, RESET);
    for (i, item) in player.inventory.iter().enumerate() {
        println!("  {}{}){} {} [{}{}{}]  {}{}",
            CYAN, i + 1, RESET,
            item.display_name(),
            item.rarity.color(), item.rarity.name(), RESET,
            DIM, item.description);
    }
    println!("  {}{}){} Cancel", CYAN, player.inventory.len() + 1, RESET);

    let input = get_input(&format!("  {}Choose item > {}", BOLD, RESET));
    let Ok(n) = input.parse::<usize>() else { return false; };

    if n == player.inventory.len() + 1 || n == 0 {
        return false;
    }
    if n > player.inventory.len() {
        println!("  {}Invalid choice.{}", RED, RESET);
        return false;
    }

    let item = player.inventory.remove(n - 1);
    apply_item(player, &item, rng);
    true
}

pub fn apply_item(player: &mut Player, item: &Item, _rng: &mut impl Rng) {
    match item.kind {
        ItemKind::Consumable => {
            if item.name.contains("Wallet") {
                // Special: gives gold
                let gold = 10;
                player.gold += gold;
                println!("  {}You rifle through the wallet and find {} gold.{}", YELLOW, gold, RESET);
            } else if item.name.contains("Dungeon Token") {
                let gold = 200;
                player.gold += gold;
                println!("  {}The dungeon tokens convert to {} gold automatically.{}", YELLOW, gold, RESET);
            } else if item.heal_amount > 0 {
                let actual = item.heal_amount.min((player.max_hp - player.hp) as u32);
                player.hp = (player.hp + item.heal_amount as i32).min(player.max_hp);
                if item.name.contains("Champions") {
                    player.max_hp += 20;
                    player.hp = player.max_hp;
                    player.mp = player.max_mp;
                    println!("  {}✨ The Vial of Champions flows through you. Max HP +20, fully restored!{}", YELLOW, RESET);
                } else {
                    println!("  {}💊 You use {}. Restored {} HP. ({}/{}){}",
                        GREEN, item.name, actual, player.hp, player.max_hp, RESET);
                }
            }
        }
        ItemKind::ManaPotion => {
            let actual = item.mana_amount.min((player.max_mp - player.mp) as u32);
            player.mp = (player.mp + item.mana_amount as i32).min(player.max_mp);
            println!("  {}✨ You use {}. Restored {} MP. ({}/{}){}",
                BLUE, item.name, actual, player.mp, player.max_mp, RESET);
        }
        ItemKind::Weapon => {
            println!("  {}You equip {}.{}", GREEN, item.name, RESET);
            if let Some(old) = player.weapon.take() {
                player.inventory.push(old);
            }
            player.weapon = Some(item.clone());
        }
        ItemKind::Armor => {
            println!("  {}You equip {}.{}", GREEN, item.name, RESET);
            if let Some(old) = player.armor.take() {
                player.inventory.push(old);
            }
            player.armor = Some(item.clone());
        }
        ItemKind::Throwable => {
            // Throwable used in item menu acts as emergency grenade
            let dmg = item.throw_dmg_max; // use max as direct damage from close use
            println!("  {}💥 You use {} directly for {} damage to yourself and anyone nearby!{}",
                YELLOW, item.name, dmg / 2, RESET);
            player.hp -= (dmg / 2) as i32;
        }
        _ => {
            println!("  {}You examine the {}. Nothing happens.{}", DIM, item.name, RESET);
        }
    }
}

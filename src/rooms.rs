// ============================================================
//  rooms.rs — Exploration, rooms, and random events
// ============================================================

use rand::Rng;
use crate::types::*;
use crate::display::*;
use crate::items::*;
use crate::combat::{run_combat, CombatResult, apply_item};
use crate::monsters::*;

pub enum FloorResult {
    Descended,
    Died,
    Quit,
}

/// Explore a floor until the player either clears the boss, dies, or quits.
pub fn explore_floor(player: &mut Player, rng: &mut impl Rng) -> FloorResult {
    announce_floor(player);

    let rooms_to_clear = match player.floor {
        1 => 6,
        2 => 7,
        _ => 8,
    };
    let miniboss_at = rooms_to_clear / 2;

    let mut rooms_done: u32 = 0;

    loop {
        print_status(player);

        if rooms_done >= rooms_to_clear {
            // Time to fight the floor boss
            match floor_boss_fight(player, rng) {
                CombatResult::Victory => {
                    post_boss(player, rng);
                    player.floor += 1;
                    if player.floor > 3 {
                        return endgame(player);
                    }
                    return FloorResult::Descended;
                }
                CombatResult::Defeat => return FloorResult::Died,
                CombatResult::Fled => {
                    println!("{}You ran from the boss. The door seals behind you — you must try again.{}",
                        RED, RESET);
                    press_enter();
                    continue;
                }
            }
        }

        // Special: miniboss on Floor 1 at the halfway point
        if player.floor == 1 && rooms_done == miniboss_at {
            if let Some(res) = miniboss_encounter(player, rng) {
                match res {
                    CombatResult::Victory => {
                        rooms_done += 1;
                        player.rooms_cleared += 1;
                        continue;
                    }
                    CombatResult::Defeat => return FloorResult::Died,
                    CombatResult::Fled => {
                        press_enter();
                        continue;
                    }
                }
            }
        }

        let options = [
            "Explore the next room",
            "Check inventory",
            "Rest briefly (risky)",
            "Quit the run",
        ];
        let choice = get_choice(
            &format!("{}You stand in a dim corridor. What do you do?{}", BOLD, RESET),
            &options);

        match choice {
            0 => {
                let result = run_room(player, rng);
                match result {
                    RoomOutcome::Cleared => {
                        rooms_done += 1;
                        player.rooms_cleared += 1;
                    }
                    RoomOutcome::Skipped => {}
                    RoomOutcome::Died => return FloorResult::Died,
                }
            }
            1 => inventory_menu(player, rng),
            2 => rest(player, rng),
            _ => {
                if confirm_quit() {
                    return FloorResult::Quit;
                }
            }
        }
    }
}

fn announce_floor(player: &Player) {
    clear_screen();
    let (name, flavor) = match player.floor {
        1 => ("FLOOR 1 — THE BASEMENT",
              "Flickering fluorescent tubes. The smell of rats and mildew. \
               Borant's opening act: a crumbling imitation of a thousand office basements, \
               stitched together with hobgoblins and worse."),
        2 => ("FLOOR 2 — THE WAREHOUSE STACKS",
              "Endless metal shelving climbs into darkness. Forklifts sit abandoned \
               in the aisles. Something moves between the pallets, and the sponsors \
               are already bidding on who finds you first."),
        3 => ("FLOOR 3 — THE FOUNDRY",
              "Heat pours off the walls. Somewhere far above, the dungeon audience \
               cheers your bleeding progress. This is where the Borant Corporation \
               stops pretending and starts grinding."),
        _ => ("DEEPER", "The dungeon hungers."),
    };

    print_box(name, &[
        flavor,
        "",
        &format!("Princess Donut is beside you, tail flicking. She is not impressed."),
    ], "cyan");

    print_system_msg(&match player.floor {
        1 => "Welcome to Floor 1, crawler. Clear the rooms. Kill the boss. \
              Survive. The viewers are watching.".to_string(),
        2 => "Floor 2 unlocked. Difficulty scaling applied. Sponsor interest: ELEVATED."
              .to_string(),
        3 => "Floor 3 unlocked. Rating adjusted to MATURE. The audience is paying \
              premium subscription rates for what happens down here.".to_string(),
        _ => "Further descent detected. The Borant Corporation reminds you: \
              there is no surface to return to.".to_string(),
    });

    press_enter();
}

enum RoomOutcome {
    Cleared,
    Skipped,
    Died,
}

fn run_room(player: &mut Player, rng: &mut impl Rng) -> RoomOutcome {
    // Weighted event table
    let roll = rng.gen_range(0..100);
    let event = match (player.floor, roll) {
        (_, 0..=9)   => Event::Monster,
        (_, 55..=69)  => Event::LootBox,
        (_, 70..=79)  => Event::Trap,
        (_, 80..=86)  => Event::Merchant,
        (_, 10..=54)  => Event::NPC,
        (_, 93..=96)  => Event::Sponsor,
        _             => Event::Flavor,
    };

    match event {
        Event::Monster  => monster_room(player, rng),
        Event::LootBox  => loot_room(player, rng),
        Event::Trap     => trap_room(player, rng),
        Event::Merchant => merchant_room(player, rng),
        Event::NPC      => npc_room(player, rng),
        Event::Sponsor  => sponsor_event(player, rng),
        Event::Flavor   => flavor_room(player, rng),
    }
}

enum Event { Monster, LootBox, Trap, Merchant, NPC, Sponsor, Flavor }

fn monster_room(player: &mut Player, rng: &mut impl Rng) -> RoomOutcome {
    let m = roll_monster(player.floor, rng);
    match run_combat(player, m, rng) {
        CombatResult::Victory => {
            // Small chance to find a loot box after combat
            if rng.gen_range(0..100) < 20 {
                println!("{}You spot a {}Crawler Box{} half-buried in the rubble.{}",
                    DIM, YELLOW, RESET, RESET);
                open_box(player, random_box_rarity(rng), rng);
            }
            RoomOutcome::Cleared
        }
        CombatResult::Fled     => RoomOutcome::Skipped,
        CombatResult::Defeat   => RoomOutcome::Died,
    }
}

fn roll_monster(floor: u32, rng: &mut impl Rng) -> Monster {
    match floor {
        1 => {
            let r = rng.gen_range(0..6);
            match r {
                0 => make_dire_rat(rng),
                1 => make_hobgoblin(rng),
                2 => make_zombie(rng),
                3 => make_feral_dog(rng),
                4 => make_dungeon_imp(rng),
                _ => make_hobgoblin_shaman(rng),
            }
        }
        2 => {
            let r = rng.gen_range(0..5);
            match r {
                0 => make_cave_troll(rng),
                1 => make_skeleton_archer(rng),
                2 => make_dungeon_crawler_competitor(rng),
                3 => make_hobgoblin(rng),
                _ => make_dungeon_imp(rng),
            }
        }
        _ => {
            let r = rng.gen_range(0..4);
            match r {
                0 => make_cave_troll(rng),
                1 => make_skeleton_archer(rng),
                2 => make_dungeon_crawler_competitor(rng),
                _ => make_hobgoblin_shaman(rng),
            }
        }
    }
}

fn loot_room(player: &mut Player, rng: &mut impl Rng) -> RoomOutcome {
    let rarity = random_box_rarity(rng);
    print_box("CRAWLER BOX DISCOVERED",
        &[
            "Sitting on a plinth in the middle of the room is a Crawler Box.",
            "Princess Donut is already trying to claim it.",
        ], "yellow");
    let choice = get_choice("Open the box?",
        &["Open it", "Leave it (Donut will be furious)"]);
    if choice == 0 {
        open_box(player, rarity, rng);
        player.boxes_opened += 1;
        player.add_viewers(15_000);
    } else {
        println!("{}Princess Donut headbutts your ankle in disgust.{}", DIM, RESET);
    }
    RoomOutcome::Cleared
}

fn open_box(player: &mut Player, rarity: Rarity, rng: &mut impl Rng) {
    let lootbox = generate_loot_box(rarity.clone(), player.floor, rng);
    let refs: Vec<&Item> = lootbox.items.iter().collect();
    print_loot_box(&rarity, &refs);
    for item in lootbox.items {
        println!("  {}Added to inventory: {}{}", DIM, item.display_name(), RESET);
        player.inventory.push(item);
    }
    // Achievement on first box opened
    if player.boxes_opened == 0 && player.unlock_achievement("First Crack at the Cardboard") {
        print_achievement("First Crack at the Cardboard");
    }
}

fn trap_room(player: &mut Player, rng: &mut impl Rng) -> RoomOutcome {
    let traps = [
        ("Pressure Plate", "A tile sinks under your foot. A dart fires from the wall."),
        ("Collapsing Floor", "The floor crumbles and you drop into the room below."),
        ("Spore Cloud", "Yellow spores burst from a crack in the ceiling."),
        ("Rusty Bear Trap", "Steel jaws snap shut around your ankle."),
    ];
    let (name, flavor) = traps[rng.gen_range(0..traps.len())];
    print_box(&format!("TRAP: {}", name), &[flavor], "red");

    let dex_save = 10 + (player.dexterity - 5);
    let roll = rng.gen_range(1..=20);
    println!("{}Dexterity save: rolled {} + {} vs DC 15{}",
        DIM, roll, dex_save - 10, RESET);

    if roll + (dex_save - 10) >= 15 {
        println!("{}You dodge clear at the last possible second.{}", GREEN, RESET);
        player.add_viewers(5_000);
    } else {
        let dmg = rng.gen_range(5..=15).max(1);
        player.hp -= dmg;
        println!("{}You take {} damage!{}", RED, dmg, RESET);
        if player.hp <= 0 {
            println!("{}A trap. In a dungeon. How embarrassing.{}", DIM, RESET);
            return RoomOutcome::Died;
        }
    }

    press_enter();
    RoomOutcome::Cleared
}

fn merchant_room(player: &mut Player, _rng: &mut impl Rng) -> RoomOutcome {
    print_box("SAFE ROOM — MR. MONGOOSE'S EMPORIUM",
        &[
            "A cramped shop appears in a bubble of safe-room calm.",
            "Behind the counter: Mr. Mongoose, a six-foot-tall cartoon mongoose",
            "in a vest, chewing on a stale cigarillo.",
            "",
            "'Spend your gold, crawler. You probably won't live to spend it later.'",
        ], "green");

    // Build inventory
    let stock: Vec<(Item, u32)> = vec![
        (Item::consumable("Medkit",
            "A proper first aid kit. Significantly restores health.",
            Rarity::Uncommon, 0, 40), 45),
        (Item::consumable("Protein Bar",
            "Stale but edible. Restores some health.", Rarity::Common, 0, 15), 10),
        (Item::mana_potion("Mana Potion",
            "A vial of glowing blue liquid. Restores MP.", 0, 15), 25),
        (Item::weapon("Fire Axe",
            "A heavy firefighter's axe. Solid choice.", Rarity::Uncommon, 0, 7, 14), 80),
        (Item::armor("Leather Jacket",
            "A biker's leather jacket. Provides decent protection.",
            Rarity::Uncommon, 0, 2), 60),
        (Item::throwable("Molotov Cocktail",
            "A glass bottle of flaming alcohol.", Rarity::Uncommon, 0, 18, 30), 35),
    ];

    loop {
        println!("\n  {}Gold: {}{}{}", YELLOW, BOLD, player.gold, RESET);
        let mut labels: Vec<String> = stock.iter()
            .map(|(it, p)| format!("{} [{}g] — {}", it.display_name(), p, it.description))
            .collect();
        labels.push("Leave the shop".to_string());
        let refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
        let choice = get_choice("What'll it be?", &refs);
        if choice == stock.len() {
            break;
        }
        let (item, price) = &stock[choice];
        if player.gold < *price {
            println!("  {}Not enough gold.{}", RED, RESET);
            continue;
        }
        player.gold -= *price;
        let mut bought = item.clone();
        bought.value = *price;
        println!("  {}You purchase {}.{}", GREEN, bought.display_name(), RESET);
        player.inventory.push(bought);
    }
    RoomOutcome::Cleared
}

fn npc_room(player: &mut Player, rng: &mut impl Rng) -> RoomOutcome {
    let kind = rng.gen_range(0..3);
    match kind {
        0 => {
            print_box("DYING CRAWLER",
                &[
                    "A man slumps against the wall, breath shallow.",
                    "'Take it... the dungeon isn't finished with me but I am.'",
                    "He presses a pouch of gold into your hand.",
                ], "magenta");
            let gold = rng.gen_range(20..=60);
            player.gold += gold;
            println!("{}Received {} gold.{}", YELLOW, gold, RESET);
            if rng.gen_range(0..100) < 40 {
                let item = random_item_for_floor_shim(Rarity::Uncommon, player.floor, rng);
                println!("{}He also drops: {}{}", DIM, item.display_name(), RESET);
                player.inventory.push(item);
            }
        }
        1 => {
            print_box("LOST MANAGER",
                &[
                    "A mid-level Borant manager named Carl-not-you is panicking.",
                    "'Listen — not all of us knew what Floor 1 would be, okay?'",
                    "He tosses you a crumpled envelope and scurries into a vent.",
                ], "magenta");
            let gold = rng.gen_range(10..=30);
            player.gold += gold;
            println!("{}Envelope contained: {} gold.{}", YELLOW, gold, RESET);
            player.add_viewers(25_000);
        }
        _ => {
            print_box("PRINCESS DONUT: A ROYAL OPINION",
                &[
                    "Princess Donut sits regally on a overturned filing cabinet,",
                    "tail curled, emerald collar gleaming. She locks eyes with you",
                    "and delivers a slow, judgmental blink.",
                    "",
                    "'Mrow.' (She approves of your progress. Barely.)",
                ], "magenta");
            // Small heal + donut heal
            let heal = 10;
            player.hp = (player.hp + heal).min(player.max_hp);
            player.donut.hp = player.donut.max_hp;
            player.donut.alive = true;
            println!("{}You both feel encouraged. (+{} HP, Donut fully healed.){}",
                GREEN, heal, RESET);
        }
    }
    press_enter();
    RoomOutcome::Cleared
}

fn sponsor_event(player: &mut Player, rng: &mut impl Rng) -> RoomOutcome {
    let sponsors = [
        ("THE SKYFATHER CONGLOMERATE",
         "A divine sponsor has noticed you. They enclose a small blessing."),
        ("GILDED HOOF EQUIPMENT",
         "A crate of gear drops from the ceiling with a faintly horse-shaped logo."),
        ("MS. VRUUUM'S CARAVAN",
         "A strange merchant-queen sends her compliments and a gift."),
        ("THE UNDER-FLOOR KOBOLD UNION",
         "A whole guild of kobolds cheer you from a safe distance. They mail you swag."),
    ];
    let (sponsor, flavor) = sponsors[rng.gen_range(0..sponsors.len())];
    print_sponsor_msg(sponsor, flavor);

    let rarity = sponsor_box_rarity(rng);
    open_box(player, rarity, rng);
    player.add_viewers(50_000);
    press_enter();
    RoomOutcome::Cleared
}

fn flavor_room(player: &mut Player, rng: &mut impl Rng) -> RoomOutcome {
    let flavors = [
        "The room is empty except for a television mounted to the wall. It is \
         playing a commercial for life insurance. The people in the ad are clearly \
         dungeon crawlers who did not make it. Princess Donut is unsettled.",
        "A dead goblin is splayed on a beanbag chair in the middle of the room. \
         Someone has placed a game controller in his hand as a joke. He is still warm.",
        "A hand-painted sign reads: 'BORANT CORPORATION — WE SEE YOU.' Underneath, \
         a second sign in smaller text reads: 'Please smile.' You do not smile.",
        "The dungeon has generated what appears to be a break room. There is a \
         watercooler. The water is fine. There is a printer. The printer is haunted.",
        "A child's drawing is taped to the wall. It shows a stick figure with a cat \
         fighting a monster. Someone has scrawled in adult handwriting: 'This is you. \
         Good luck.'",
    ];
    let pick = flavors[rng.gen_range(0..flavors.len())];
    print_box("AN EMPTY ROOM", &[pick], "cyan");

    // Small reward
    let gold = rng.gen_range(2..=6);
    player.gold += gold;
    player.add_viewers(2_000);
    println!("{}You take a breath. (+{} gold, viewers noted){}", DIM, gold, RESET);
    press_enter();
    RoomOutcome::Cleared
}

fn rest(player: &mut Player, rng: &mut impl Rng) -> () {
    println!();
    if rng.gen_range(0..100) < 35 {
        println!("{}You try to rest. Something was watching. You hear skittering.{}",
            RED, RESET);
        press_enter();
        let m = roll_monster(player.floor, rng);
        let _ = run_combat(player, m, rng);
    } else {
        let hp_heal = 10 + (player.constitution - 5).max(0);
        let mp_heal = if player.max_mp > 0 { 5 } else { 0 };
        player.hp = (player.hp + hp_heal).min(player.max_hp);
        player.mp = (player.mp + mp_heal).min(player.max_mp);
        player.donut.hp = player.donut.max_hp;
        player.donut.alive = true;
        println!("{}You take a short rest in a shadowed alcove. (+{} HP, +{} MP, Donut restored){}",
            GREEN, hp_heal, mp_heal, RESET);
    }
    press_enter();
}

fn miniboss_encounter(player: &mut Player, rng: &mut impl Rng) -> Option<CombatResult> {
    if player.floor != 1 { return None; }
    print_system_msg(
        "Mid-floor miniboss detected: STUMBLES. \
         A named zombie — killing him is worth a significant viewer bonus.");
    press_enter();
    let mut mb = make_floor1_miniboss(rng);
    mb.hp = mb.max_hp;
    let res = run_combat(player, mb, rng);
    if matches!(res, CombatResult::Victory) {
        player.add_viewers(250_000);
        if player.unlock_achievement("He Was Already Dead") {
            print_achievement("He Was Already Dead");
        }
    }
    Some(res)
}

fn floor_boss_fight(player: &mut Player, rng: &mut impl Rng) -> CombatResult {
    let boss = match player.floor {
        1 => make_floor1_boss(rng),
        2 => make_floor2_boss(rng),
        _ => make_floor3_boss(rng),
    };
    print_system_msg(&format!(
        "FLOOR {} BOSS ENCOUNTER. All corridors have sealed. \
         The viewers are peaking. The sponsors are betting. \
         Good luck, crawler.", player.floor));
    press_enter();
    run_combat(player, boss, rng)
}

fn post_boss(player: &mut Player, rng: &mut impl Rng) {
    let ach = match player.floor {
        1 => "Floor 1 Cleared — Still Breathing",
        2 => "Floor 2 Cleared — Warehouse Warrior",
        _ => "Floor 3 Cleared — Furnace-Forged",
    };
    if player.unlock_achievement(ach) {
        print_achievement(ach);
    }
    player.add_viewers(rng.gen_range(500_000..=1_500_000));
    print_system_msg("Descending to the next floor. Safe-room checkpoint reached.");
    press_enter();
}

fn endgame(player: &mut Player) -> FloorResult {
    clear_screen();
    print_box("THE STAIRS DOWN", &[
        "You stand at the lip of a staircase that descends beyond sight.",
        "Warm air rises. Somewhere below, Floor 4 is still being built.",
        "",
        "For the first time since the surface ended, you and Princess Donut",
        "are alone in a quiet place. She rubs against your leg. You almost cry.",
        "",
        "But the Borant Corporation never sleeps, and neither, anymore, do you.",
    ], "yellow");

    print_system_msg(&format!(
        "CRAWLER {} has cleared three floors. Current viewers: {}. \
         This saga continues... but this chapter ends here.",
        player.name,
        fmt_num(player.viewers)));

    FloorResult::Quit
}

fn confirm_quit() -> bool {
    let c = get_choice(
        &format!("{}Quit the run? Princess Donut will NOT forgive you.{}", RED, RESET),
        &["Stay", "Quit"]);
    c == 1
}

fn inventory_menu(player: &mut Player, rng: &mut impl Rng) {
    loop {
        println!();
        sep();
        println!("  {}{}INVENTORY{}  ({} items, {}g){}",
            BOLD, CYAN, RESET, player.inventory.len(), player.gold, RESET);
        sep();
        if player.inventory.is_empty() {
            println!("  {}(empty){}", DIM, RESET);
            press_enter();
            return;
        }
        for (i, it) in player.inventory.iter().enumerate() {
            println!("  {}{}){} {} {}[{}]{}  {}{}{}",
                CYAN, i + 1, RESET,
                it.display_name(),
                DIM, it.rarity.name(), RESET,
                DIM, it.description, RESET);
        }
        println!("  {}{}){} Back", CYAN, player.inventory.len() + 1, RESET);

        let input = get_input(&format!("  {}Use / equip > {}", BOLD, RESET));
        let Ok(n) = input.parse::<usize>() else { return; };
        if n == 0 || n > player.inventory.len() { return; }
        let item = player.inventory.remove(n - 1);
        apply_item(player, &item, rng);
        press_enter();
    }
}

/// A thin wrapper so rooms.rs can request a random item without re-importing
/// the private function from items.rs.
fn random_item_for_floor_shim(rarity: Rarity, floor: u32, rng: &mut impl Rng) -> Item {
    let lb = generate_loot_box(rarity, floor, rng);
    lb.items.into_iter().next().unwrap_or_else(|| Item::consumable(
        "Dungeon Ration", "A dry ration from who-knows-where.",
        Rarity::Common, 2, 8))
}

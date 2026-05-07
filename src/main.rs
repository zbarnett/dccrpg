// ============================================================
//  main.rs — Entry point, intro sequence, and run loop
// ============================================================

mod types;
mod display;
mod items;
mod monsters;
mod combat;
mod rooms;

use rand::thread_rng;
use crate::types::*;
use crate::display::*;
use crate::rooms::{explore_floor, FloorResult};

#[cfg(windows)]
fn enable_ansi() {
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    unsafe {
        let h = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut mode: u32 = 0;
        if GetConsoleMode(h, &mut mode) != 0 {
            SetConsoleMode(h, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

#[cfg(not(windows))]
fn enable_ansi() {}

fn main() {
    enable_ansi();
    print_title();
    let mut rng = thread_rng();

    let name = pick_name();
    let mut player = Player::new(&name);

    intro_sequence(&mut player);
    choose_class(&mut player);

    loop {
        match explore_floor(&mut player, &mut rng) {
            FloorResult::Descended => continue,
            FloorResult::Died      => { death_screen(&player); break; }
            FloorResult::Quit      => { end_screen(&player);  break; }
        }
    }
}

fn pick_name() -> String {
    clear_screen();
    println!();
    print_box("CRAWLER REGISTRATION",
        &[
            "The Borant Corporation needs your name for the leaderboard.",
            "(Leave blank to be registered as 'Carl'.)",
        ], "cyan");
    let n = get_input(&format!("{}Your name > {}", BOLD, RESET));
    if n.is_empty() { "Carl".to_string() } else { n }
}

fn intro_sequence(player: &mut Player) {
    clear_screen();
    print_box("THE COLLAPSE",
        &[
            "One ordinary morning, the buildings disappear.",
            "Not destroyed. Not collapsed. Simply gone, like a stage crew",
            "finally striking an unwanted set.",
            "",
            "The ground opens. Nine floors of dungeon appear beneath what was,",
            "hours earlier, your apartment building. A cheerful announcement",
            "rolls across every remaining speaker on the planet:",
            "",
            "'Welcome to the dungeon. Descend or die.'",
        ], "red");
    press_enter();

    print_box(&format!("{}, AND A CAT", player.name.to_uppercase()),
        &[
            "You barely make it out in boxer shorts and a flannel jacket.",
            "Princess Donut — your ex-girlfriend's prize-winning show cat —",
            "is yowling from under the couch when the floor opens.",
            "",
            "You grab her. The world grabs you.",
            "",
            "You both land on Floor 1 together.",
        ], "magenta");
    press_enter();

    print_system_msg(
        "The Borant Corporation welcomes new Crawler to Dungeon #2,071. \
         You are entertainment. Behave accordingly.");
    press_enter();

    print_sponsor_msg("A CURIOUS OBSERVER",
        "'Oh my. A man in boxers and a talking cat. The audience is \
         going to love you. Here — a starter gift, on the house.'");
    player.gold += 15;
    player.add_viewers(50_000);
    press_enter();
}

fn choose_class(player: &mut Player) {
    clear_screen();
    print_box("CLASS SELECTION",
        &[
            "You've survived long enough to choose a class.",
            "Borant's class system measures how you fight — and how you die.",
            "Princess Donut is already a Noble Cat. She cannot be reclassed.",
        ], "cyan");

    let classes = [
        Class::Punk,
        Class::Hooligan,
        Class::Skirmisher,
        Class::Sapper,
        Class::ApprenticeWizard,
    ];

    loop {
        for (i, c) in classes.iter().enumerate() {
            println!("  {}{}){} {}{}{}", CYAN, i + 1, RESET, BOLD, c.name(), RESET);
            println!("     {}{}{}", DIM, c.description(), RESET);
        }
        println!("  {}{}){} Review again", CYAN, classes.len() + 1, RESET);

        let input = get_input(&format!("\n{}Choose class > {}", BOLD, RESET));
        let Ok(n) = input.parse::<usize>() else { continue; };
        if n == 0 || n > classes.len() + 1 { continue; }
        if n == classes.len() + 1 { continue; }

        let picked = classes[n - 1].clone();
        println!();
        println!("{}Confirm: {}{}{}?", YELLOW, BOLD, picked.name(), RESET);
        let c = get_choice("", &["Yes, lock it in", "No, go back"]);
        if c == 0 {
            player.apply_class(picked);
            print_system_msg(&format!(
                "Class registered: {}. Stats and starter gear applied.",
                player.class.name()));
            press_enter();
            return;
        }
    }
}

fn death_screen(player: &Player) {
    clear_screen();
    print_box("CRAWLER DECEASED", &[
        "The Borant Corporation thanks you for your service.",
        "",
        &format!("{} fell on Floor {}.", player.name, player.floor),
        &format!("Kills: {}   Rooms cleared: {}   Boxes opened: {}",
            player.kills, player.rooms_cleared, player.boxes_opened),
        &format!("Final viewers: {}", fmt_num(player.viewers)),
        "",
        "Princess Donut is furious. Somewhere, in another dungeon,",
        "she is already plotting your sequel.",
    ], "red");
}

fn end_screen(player: &Player) {
    clear_screen();
    print_box("END OF RUN", &[
        &format!("{}, Crawler — Class {}, Level {}",
            player.name, player.class.name(), player.level),
        &format!("Final floor reached: {}", player.floor),
        &format!("Kills: {}    Rooms: {}    Boxes: {}",
            player.kills, player.rooms_cleared, player.boxes_opened),
        &format!("Gold: {}", player.gold),
        &format!("Viewers: {}", fmt_num(player.viewers)),
        &format!("Achievements: {}", player.achievements.len()),
    ], "yellow");

    for a in &player.achievements {
        println!("  {}★{} {}", YELLOW, RESET, a);
    }
    println!();
    println!("{}Thanks for crawling.{}", DIM, RESET);
}

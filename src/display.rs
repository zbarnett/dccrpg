// ============================================================
//  display.rs — All UI/display functions
// ============================================================

use std::io::{self, Write};
use crate::types::*;

pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();
}

pub fn press_enter() {
    print!("\n{}[Press ENTER to continue...]{}  ", DIM, RESET);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
}

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn get_choice(prompt: &str, options: &[&str]) -> usize {
    loop {
        println!("\n{}", prompt);
        for (i, opt) in options.iter().enumerate() {
            println!("  {}{}){} {}", CYAN, i + 1, RESET, opt);
        }
        let input = get_input(&format!("{}Choice > {}", BOLD, RESET));
        if let Ok(n) = input.parse::<usize>() {
            if n >= 1 && n <= options.len() {
                return n - 1;
            }
        }
        println!("{}Please enter a number between 1 and {}.{}", RED, options.len(), RESET);
    }
}

pub fn sep() {
    println!("{}────────────────────────────────────────────────────{}", DIM, RESET);
}

pub fn dsep() {
    println!("{}════════════════════════════════════════════════════{}", CYAN, RESET);
}

fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut esc = false;
    for c in s.chars() {
        if c == '\x1b' { esc = true; }
        else if esc && c == 'm' { esc = false; }
        else if !esc { out.push(c); }
    }
    out
}

fn pad_to(s: &str, width: usize) -> String {
    let vis = strip_ansi(s).len();
    if vis >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - vis))
    }
}

pub fn print_box(title: &str, lines: &[&str], color: &str) {
    let inner = 52usize;
    let (col, _) = match color {
        "red"     => (RED, ()),
        "green"   => (GREEN, ()),
        "yellow"  => (YELLOW, ()),
        "cyan"    => (CYAN, ()),
        "magenta" => (MAGENTA, ()),
        _         => (WHITE, ()),
    };

    let title_pad = (inner.saturating_sub(title.len())) / 2;
    println!();
    println!("{}╔{}╗{}", col, "═".repeat(inner), RESET);
    println!("{}║{}{}{}{}{}{}{}║{}",
        col, RESET,
        " ".repeat(title_pad),
        BOLD, title, RESET,
        " ".repeat(inner.saturating_sub(title.len() + title_pad)),
        col, RESET);
    println!("{}╠{}╣{}", col, "═".repeat(inner), RESET);
    for line in lines {
        let content = format!("  {}", line);
        let vis_len = strip_ansi(&content).len();
        let padding = inner.saturating_sub(vis_len);
        println!("{}║{}{}{}{}║{}",
            col, RESET, content,
            " ".repeat(padding),
            col, RESET);
    }
    println!("{}╚{}╝{}", col, "═".repeat(inner), RESET);
    println!();
}

pub fn print_achievement(name: &str) {
    println!();
    println!("{}╔══════════════════════════════════════════════════╗{}", YELLOW, RESET);
    println!("{}║          ★  ACHIEVEMENT UNLOCKED!  ★             ║{}", YELLOW, RESET);
    let padded = pad_to(&format!("{}{}{}  ", BOLD, name, RESET), 50);
    println!("{}║  {}║{}", YELLOW, padded, RESET);
    println!("{}╚══════════════════════════════════════════════════╝{}", YELLOW, RESET);
    println!();
}

pub fn print_system_msg(msg: &str) {
    println!();
    println!("{}╔══════════════════════════════════════════════════╗{}", CYAN, RESET);
    println!("{}║  📡  BORANT CORPORATION — SYSTEM NOTIFICATION    ║{}", CYAN, RESET);
    let padded = pad_to(&format!("{}  {}{}  ", WHITE, msg, RESET), 52);
    println!("{}║{}║{}", CYAN, padded, RESET);
    println!("{}╚══════════════════════════════════════════════════╝{}", CYAN, RESET);
    println!();
}

pub fn print_sponsor_msg(sponsor: &str, msg: &str) {
    println!();
    println!("{}╔══════════════════════════════════════════════════╗{}", MAGENTA, RESET);
    let s_line = format!("  ✨ SPONSOR: {}{}{}", YELLOW, sponsor, RESET);
    let padded_s = pad_to(&s_line, 52);
    println!("{}║{}║{}", MAGENTA, padded_s, RESET);
    let m_line = format!("  {}", msg);
    let padded_m = pad_to(&m_line, 52);
    println!("{}║{}║{}", MAGENTA, padded_m, RESET);
    println!("{}╚══════════════════════════════════════════════════╝{}", MAGENTA, RESET);
    println!();
}

pub fn print_viewers(viewers: u64) {
    println!("{}  📺  Live Viewers: {}{}{}{}", DIM, YELLOW, fmt_num(viewers), RESET, RESET);
}

pub fn fmt_num(n: u64) -> String {
    if n >= 1_000_000_000 { format!("{:.2}B", n as f64 / 1e9) }
    else if n >= 1_000_000 { format!("{:.2}M", n as f64 / 1e6) }
    else if n >= 1_000     { format!("{:.1}K", n as f64 / 1e3) }
    else                   { n.to_string() }
}

pub fn print_status(player: &Player) {
    let bar = |val: i32, max: i32, width: usize, col: &str| -> String {
        let filled = ((val as f64 / max as f64) * width as f64) as usize;
        format!("{}{}{}{}",
            col, "█".repeat(filled.min(width)),
            "░".repeat(width.saturating_sub(filled)),
            RESET)
    };
    let hp_col = if player.hp <= player.max_hp / 4 { RED }
                 else if player.hp <= player.max_hp / 2 { YELLOW }
                 else { GREEN };

    dsep();
    println!("  {}{}{} │ {} │ {}Lv.{}{} │ {}Floor {}{} │ {}{}g{}",
        BOLD, player.name, RESET,
        player.class.name(),
        CYAN, player.level, RESET,
        YELLOW, player.floor, RESET,
        YELLOW, player.gold, RESET);
    sep();
    println!("  {}HP{} {}{:>3}/{:<3}{} {}",
        RED, RESET,
        hp_col, player.hp, player.max_hp, RESET,
        bar(player.hp, player.max_hp, 20, hp_col));
    if player.max_mp > 0 {
        println!("  {}MP{} {}{:>3}/{:<3}{} {}",
            BLUE, RESET,
            BLUE, player.mp, player.max_mp, RESET,
            bar(player.mp, player.max_mp, 20, BLUE));
    }
    println!("  {}EXP{} {}/{}", CYAN, RESET, player.exp, player.exp_to_next);

    let weapon_name = player.weapon.as_ref().map(|w| w.name.as_str()).unwrap_or("Unarmed");
    let armor_name  = player.armor.as_ref().map(|a| a.name.as_str()).unwrap_or("None");
    println!("  {}Weapon:{} {}  │  {}Armor:{} {}",
        YELLOW, RESET, weapon_name,
        YELLOW, RESET, armor_name);

    if player.class == Class::Sapper {
        println!("  {}Pipe Bombs:{} {}", YELLOW, RESET, player.pipe_bombs);
    }

    // Donut
    if player.donut.alive {
        let d = &player.donut;
        println!("  🐱 {} [{}{}{}] Lv.{}  HP {}{}/{}{} {}",
            d.name,
            MAGENTA, d.class, RESET,
            d.level,
            GREEN, d.hp, d.max_hp, RESET,
            bar(d.hp, d.max_hp, 12, GREEN));
    } else {
        println!("  🐱 Princess Donut — {}INCAPACITATED{}", RED, RESET);
    }

    println!("  📺 {}Viewers:{} {}   │  {}Kills:{} {}   │  {}Boxes:{} {}",
        MAGENTA, RESET, fmt_num(player.viewers),
        RED, RESET, player.kills,
        YELLOW, RESET, player.boxes_opened);
    dsep();
    println!();
}

pub fn print_combat_hud(player: &Player, monster: &Monster) {
    let bar = |val: i32, max: i32, width: usize, col: &str| -> String {
        let filled = ((val as f64 / max as f64) * width as f64) as usize;
        format!("{}{}{}{}",
            col, "█".repeat(filled.min(width)),
            "░".repeat(width.saturating_sub(filled)),
            RESET)
    };
    let p_col = if player.hp <= player.max_hp / 4 { RED }
                else if player.hp <= player.max_hp / 2 { YELLOW }
                else { GREEN };
    let m_col = if monster.hp <= monster.max_hp / 4 { RED }
                else if monster.hp <= monster.max_hp / 2 { YELLOW }
                else { GREEN };

    println!();
    println!("{}╔══ COMBAT ══════════════════════════════════════════╗{}", YELLOW, RESET);
    println!("{}║  {}{}{} (Lv.{})  HP {}{}/{}{} {}{}  ║{}",
        YELLOW, BOLD, player.name, RESET, player.level,
        p_col, player.hp, player.max_hp, RESET,
        bar(player.hp, player.max_hp, 12, p_col),
        YELLOW, RESET);
    if player.max_mp > 0 {
        println!("{}║  MP: {}/{:<3}{}  ║{}",
            YELLOW, player.mp, player.max_mp, YELLOW, RESET);
    }
    if player.donut.alive {
        let d = &player.donut;
        println!("{}║  🐱 Donut (Lv.{})  HP {}{}/{}{} {}{}  ║{}",
            YELLOW, d.level,
            GREEN, d.hp, d.max_hp, RESET,
            bar(d.hp, d.max_hp, 10, GREEN),
            YELLOW, RESET);
    }
    println!("{}║                 ─ vs ─                            ║{}", YELLOW, RESET);
    println!("{}║  {}{}{} (Lv.{})  HP {}{}/{}{} {}{}  ║{}",
        YELLOW, RED, monster.name, RESET, monster.level,
        m_col, monster.hp, monster.max_hp, RESET,
        bar(monster.hp, monster.max_hp, 12, m_col),
        YELLOW, RESET);
    println!("{}╚════════════════════════════════════════════════════╝{}", YELLOW, RESET);
    println!();
}

pub fn print_loot_box(rarity: &Rarity, items: &[&Item]) {
    let col = rarity.color();
    println!();
    println!("{}╔══ {} CRAWLER BOX OPENED! ══╗{}", col, rarity.name().to_uppercase(), RESET);
    for item in items {
        println!("{}║  {} + {}{}  {}║{}",
            col, item.rarity.color(), item.name, RESET, col, RESET);
    }
    println!("{}╚══════════════════════════════════════╝{}", col, RESET);
    println!();
}

pub fn print_title() {
    clear_screen();
    println!();
    println!("{}{}{}",
        YELLOW,
        r#"
  ██████╗ ██╗   ██╗███╗   ██╗ ██████╗ ███████╗ ██████╗ ███╗   ██╗
  ██╔══██╗██║   ██║████╗  ██║██╔════╝ ██╔════╝██╔═══██╗████╗  ██║
  ██║  ██║██║   ██║██╔██╗ ██║██║  ███╗█████╗  ██║   ██║██╔██╗ ██║
  ██║  ██║██║   ██║██║╚██╗██║██║   ██║██╔══╝  ██║   ██║██║╚██╗██║
  ██████╔╝╚██████╔╝██║ ╚████║╚██████╔╝███████╗╚██████╔╝██║ ╚████║
  ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝ ╚═════╝ ╚═╝  ╚═══╝"#,
        RESET);
    println!("{}{}                    C R A W L E R   C A R L{}", CYAN, BOLD, RESET);
    println!("{}              A Text-Based Dungeon Survival Experience{}", DIM, RESET);
    println!();
    println!("{}      Based on the novel series by Matt Dinniman{}", DIM, RESET);
    println!();
    println!("{}                    ─ PRESS ENTER ─{}", YELLOW, RESET);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
}

use console_lib::{keys, Color, Console};
use std::cmp::Ordering;
use std::mem;
use std::str::FromStr;
use std::time::SystemTime;
use dialog::DialogYesNo;
use crate::game::{Game, GameState};
use crate::game::level::{Level, LevelPack, Tile};
use crate::game::screen::dialog::{DialogOk, DialogSelection};

pub mod dialog;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ScreenId {
    StartMenu,

    SelectLevelPack,
    SelectLevel,

    InGame,

    SelectLevelPackEditor,
    LevelPackEditor,
    LevelEditor,
}

#[allow(unused_variables)]
pub trait Screen {
    fn draw(&self, game_state: &GameState, console: &Console);

    fn update(&mut self, game_state: &mut GameState) {}

    fn on_key_pressed(&mut self, game_state: &mut GameState, key: i32) {}
    fn on_mouse_pressed(&mut self, game_state: &mut GameState, column: usize, row: usize) {}

    fn on_dialog_selection(&mut self, game_state: &mut GameState, selection: DialogSelection) {}

    fn on_continue(&mut self, game_state: &mut GameState) {}
    fn on_set_screen(&mut self, game_state: &mut GameState) {}
}

pub struct ScreenStartMenu {}

impl ScreenStartMenu {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for ScreenStartMenu {
    fn draw(&self, _: &GameState, console: &Console) {//Draw border (top)
        console.set_color(Color::White, Color::Blue);
        console.draw_text(
            "/------------------------------------------------------------------------\\\n"
        );

        //Draw text
        console.set_color(Color::LightYellow, Color::Default);
        console.draw_text(
            "                -----------------------------------------\n                \
            .---- .---. |  ./ .---. .--.  .---. .   .\n                |     \
            |   | | /'  |   | |   : |   | |\\  |\n                '---. |   | :\
            {    |   | +---+ +---+ | \\ |\n                    | |   | | \\.  |   \
            | |   : |   | |  \\|\n                ----' '---' |  '\\ '---' '--'  \
            |   | '   '\n                ---------------------------------------\
            --\n\n\n\n\n\n-------------------------------------------------------\
            ------------------"
        );

        //Draw infos
        console.reset_color();
        let version = "Version: ".to_string() + Game::VERSION;
        console.set_cursor_pos(
            Game::CONSOLE_MIN_WIDTH - version.chars().count() - 3,
            14
        );
        console.draw_text(&version);

        console.set_cursor_pos(21, 16);
        console.draw_text("Press ");
        console.set_color(Color::LightRed, Color::Default);
        console.draw_text("ENTER");
        console.reset_color();
        console.draw_text(" to start the game!");

        console.set_cursor_pos(1, 21);
        console.draw_text("By ");
        console.set_color(Color::Default, Color::Yellow);
        console.draw_text("JDDev0");

        console.reset_color();
        console.set_cursor_pos(65, 21);
        console.draw_text("Help: ");
        console.set_color(Color::LightRed, Color::Default);
        console.draw_text("F1");

        //Draw border
        console.set_color(Color::White, Color::Blue);
        for i in 1..Game::CONSOLE_MIN_HEIGHT - 1 {
            console.set_cursor_pos(0, i);
            console.draw_text("|");

            console.set_cursor_pos(Game::CONSOLE_MIN_WIDTH - 1, i);
            console.draw_text("|");
        }
        console.draw_text("\n\\------------------------------------------------------------------------/");
    }

    fn on_key_pressed(&mut self, game_state: &mut GameState, key: i32) {
        if key == keys::ESC {
            game_state.open_dialog(DialogYesNo::new("Exit game?"));

            return;
        }

        if key == keys::F1 {
            game_state.open_help_page();

            return;
        }

        if key == keys::ENTER {
            game_state.set_screen(ScreenId::SelectLevelPack);
        }
    }

    fn on_mouse_pressed(&mut self, game_state: &mut GameState, column: usize, row: usize) {
        if row == 16 && column > 26 && column < 32 {
            self.on_key_pressed(game_state, keys::ENTER);
        }

        if row == 21 && column > 64 && column < 73 {
            game_state.open_help_page();
        }
    }

    fn on_dialog_selection(&mut self, game_state: &mut GameState, selection: DialogSelection) {
        if selection == DialogSelection::Yes {
            game_state.exit();
        }
    }
}

pub struct ScreenSelectLevelPack {}

impl ScreenSelectLevelPack {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for ScreenSelectLevelPack {
    fn draw(&self, game_state: &GameState, console: &Console) {
        console.reset_color();
        console.set_underline(true);
        console.draw_text("Select a level pack:");
        console.set_underline(false);

        //Include Level Pack Editor entry
        let entry_count = game_state.get_level_pack_count() + 1;

        //Draw first line
        console.set_cursor_pos(0, 1);
        console.draw_text("-");
        let mut max = entry_count%24;
        if entry_count/24 > 0 {
            max = 24;
        }

        for i in 0..max  {
            let x = 1 + (i%24)*3;

            console.set_cursor_pos(x, 1);
            console.draw_text("---");
        }

        for i in 0..entry_count {
            let x = 1 + (i%24)*3;
            let y = 2 + (i/24)*2;

            //First box
            if x == 1 {
                console.set_cursor_pos(x - 1, y);
                console.draw_text("|");

                console.set_cursor_pos(x - 1, y + 1);
                console.draw_text("-");
            }

            console.set_cursor_pos(x, y);
            if i == game_state.get_level_pack_count() {
                //Level Pack Editor entry
                console.set_color(Color::White, Color::LightBlue);
                console.draw_text(" +");
            }else {
                console.set_color(Color::Black, if game_state.level_packs().get(i).
                        unwrap().level_pack_best_moves_sum().is_some() {
                    Color::Green
                }else {
                    Color::Yellow
                });
                console.draw_text(format!("{:2}", i + 1));
            }

            console.reset_color();
            console.draw_text("|");

            console.set_cursor_pos(x, y + 1);
            console.draw_text("---");
        }

        //Mark selected level
        let x = (game_state.get_level_pack_index()%24)*3;
        let y = 1 + (game_state.get_level_pack_index()/24)*2;

        console.set_color(Color::Cyan, Color::Default);
        console.set_cursor_pos(x, y);
        console.draw_text("----");
        console.set_cursor_pos(x, y + 1);
        console.draw_text("|");
        console.set_cursor_pos(x + 3, y + 1);
        console.draw_text("|");
        console.set_cursor_pos(x, y + 2);
        console.draw_text("----");

        //Draw border for best time and best moves
        let y = 4 + (entry_count/24)*2;

        console.set_cursor_pos(0, y);
        console.set_color(Color::Cyan, Color::Default);
        console.draw_text(".-----------------------------------.");
        for i in 1..4 {
            console.set_cursor_pos(0, y + i);
            console.draw_text("|                                   |");
        }
        console.set_cursor_pos(0, y + 4);
        console.draw_text("\'-----------------------------------\'");
        console.reset_color();

        if game_state.get_level_pack_index() == game_state.get_level_pack_count() {
            //Level Pack Editor entry
            console.set_cursor_pos(5, y + 2);
            console.draw_text("Create or edit level packs");
        }else {
            //Draw sum of best time and sum of best moves
            console.set_cursor_pos(1, y + 1);
            console.draw_text(format!("Selected level pack: {:>14}", game_state.level_packs().get(game_state.get_level_pack_index()).unwrap().id()));
            console.set_cursor_pos(1, y + 2);
            console.draw_text("Sum of best time   : ");
            match game_state.get_current_level_pack().as_ref().unwrap().level_pack_best_time_sum() {
                None => console.draw_text("X:XX:XX:XX.XXX"),
                Some(best_time_sum) => {
                    console.draw_text(format!(
                        "{:01}:{:02}:{:02}:{:02}.{:03}",
                        best_time_sum/86400000,
                        (best_time_sum/3600000)%24,
                        (best_time_sum/60000)%60,
                        (best_time_sum/1000)%60,
                        best_time_sum%1000
                    ));
                },
            }
            console.set_cursor_pos(1, y + 3);
            console.draw_text("Sum of best moves  :        ");
            match game_state.get_current_level_pack().as_ref().unwrap().level_pack_best_moves_sum() {
                None => console.draw_text("XXXXXXX"),
                Some(best_moves_sum) => console.draw_text(format!("{:07}", best_moves_sum)),
            }
        }
    }

    fn on_key_pressed(&mut self, game_state: &mut GameState, key: i32) {
        if key == keys::ESC {
            game_state.set_screen(ScreenId::StartMenu);

            return;
        }

        if key == keys::F1 {
            game_state.open_help_page();

            return;
        }

        'outer: {
            //Include Level Pack Editor entry
            let entry_count = game_state.get_level_pack_count() + 1;

            match key {
                keys::LEFT => {
                    if game_state.current_level_pack_index == 0 {
                        break 'outer;
                    }

                    game_state.current_level_pack_index -= 1;
                },
                keys::UP => {
                    if game_state.current_level_pack_index <= 24 {
                        break 'outer;
                    }

                    game_state.current_level_pack_index -= 24;
                },
                keys::RIGHT => {
                    if game_state.current_level_pack_index + 1 >= entry_count {
                        break 'outer;
                    }

                    game_state.current_level_pack_index += 1;
                },
                keys::DOWN => {
                    if game_state.current_level_pack_index + 24 >= entry_count {
                        break 'outer;
                    }

                    game_state.current_level_pack_index += 24;
                },

                keys::ENTER => {
                    if game_state.get_level_pack_index() == game_state.get_level_pack_count() {
                        //Level Pack Editor entry
                        game_state.set_level_index(0);

                        game_state.set_screen(ScreenId::SelectLevelPackEditor);
                    }else {
                        //Set selected level
                        let min_level_not_completed = game_state.get_current_level_pack().as_ref().unwrap().min_level_not_completed();
                        if min_level_not_completed >= game_state.get_current_level_pack().as_ref().unwrap().level_count() {
                            game_state.set_level_index(0);
                        }else {
                            game_state.set_level_index(min_level_not_completed);
                        }

                        game_state.set_screen(ScreenId::SelectLevel);
                    }
                },

                _ => {},
            }
        }
    }

    fn on_mouse_pressed(&mut self, game_state: &mut GameState, column: usize, row: usize) {
        if row == 0 {
            return;
        }
        //Include Level Pack Editor entry
        let entry_count = game_state.get_level_pack_count() + 1;

        let level_pack_index = column/3 + (row - 1)/2*24;
        if level_pack_index < entry_count {
            game_state.set_level_pack_index(level_pack_index);
            self.on_key_pressed(game_state, keys::ENTER);
        }
    }
}

pub struct ScreenSelectLevel {
    selected_level: usize,
}

impl ScreenSelectLevel {
    pub fn new() -> Self {
        Self {
            selected_level: Default::default(),
        }
    }
}

impl Screen for ScreenSelectLevel {
    fn draw(&self, game_state: &GameState, console: &Console) {
        console.reset_color();
        console.set_underline(true);
        console.draw_text(format!("Select a level (Level pack \"{}\"):", game_state.get_current_level_pack().unwrap().id()));
        console.set_underline(false);

        let level_count = game_state.get_current_level_pack().as_ref().unwrap().level_count();

        //Draw first line
        console.set_cursor_pos(0, 1);
        console.draw_text("-");
        let mut max = level_count%24;
        if level_count/24 > 0 {
            max = 24;
        }
        for i in 0..max {
            let x = 1 + (i%24)*3;

            console.set_cursor_pos(x, 1);
            console.draw_text("---");
        }

        for i in 0..level_count {
            let x = 1 + (i%24)*3;
            let y = 2 + (i/24)*2;

            //First box
            if x == 1 {
                console.set_cursor_pos(x - 1, y);
                console.draw_text("|");

                console.set_cursor_pos(x - 1, y + 1);
                console.draw_text("-");
            }

            let min_level_not_completed = game_state.get_current_level_pack().as_ref().unwrap().min_level_not_completed();
            console.set_color(
                Color::Black,
                match i.cmp(&min_level_not_completed) {
                    Ordering::Less => Color::Green,
                    Ordering::Equal => Color::Yellow,
                    Ordering::Greater => Color::Red,
                }
            );
            console.set_cursor_pos(x, y);

            if i + 1 < 100 {
                console.draw_text(format!("{:2}", i + 1));
            }else {
                console.draw_text(format!("{}", (b'A' + (i as u8 + 1 - 100) / 10) as char));
                console.draw_text(format!("{}", (i + 1) % 10));
            }

            console.reset_color();
            console.draw_text("|");

            console.set_cursor_pos(x, y + 1);
            console.draw_text("---");
        }

        //Mark selected level
        let x = (self.selected_level%24)*3;
        let y = 1 + (self.selected_level/24)*2;

        console.set_color(Color::Cyan, Color::Default);
        console.set_cursor_pos(x, y);
        console.draw_text("----");
        console.set_cursor_pos(x, y + 1);
        console.draw_text("|");
        console.set_cursor_pos(x + 3, y + 1);
        console.draw_text("|");
        console.set_cursor_pos(x, y + 2);
        console.draw_text("----");

        //Draw border for best time and best moves
        let y = 4 + ((level_count - 1)/24)*2;

        console.set_cursor_pos(0, y);
        console.set_color(Color::Cyan, Color::Default);
        console.draw_text(".-------------------------.");
        for i in 1..4 {
            console.set_cursor_pos(0, y + i);
            console.draw_text("|                         |");
        }
        console.set_cursor_pos(0, y + 4);
        console.draw_text("\'-------------------------\'");

        //Draw best time and best moves
        console.reset_color();
        console.set_cursor_pos(1, y + 1);
        console.draw_text("Selected level:        ");
        let selected_level = self.selected_level;
        if selected_level + 1 < 100 {
            console.draw_text(format!("{:02}", selected_level + 1));
        }else {
            console.draw_text(format!("{}", (b'A' + (selected_level as u8 + 1 - 100) / 10) as char));
            console.draw_text(format!("{}", (selected_level + 1) % 10));
        }
        console.set_cursor_pos(1, y + 2);
        console.draw_text("Best time     : ");
        match game_state.get_current_level_pack().as_ref().unwrap().levels().get(selected_level).unwrap().best_time() {
            None => console.draw_text("XX:XX.XXX"),
            Some(best_time) => {
                console.draw_text(format!(
                    "{:02}:{:02}.{:03}",
                    best_time/60000,
                    (best_time%60000)/1000,
                    best_time%1000
                ));
            },
        }
        console.set_cursor_pos(1, y + 3);
        console.draw_text("Best moves    :      ");
        match game_state.get_current_level_pack().as_ref().unwrap().levels().get(selected_level).unwrap().best_moves() {
            None => console.draw_text("XXXX"),
            Some(best_moves) => {
                console.draw_text(format!("{:04}", best_moves));
            },
        }
    }

    fn on_key_pressed(&mut self, game_state: &mut GameState, key: i32) {
        if key == keys::ESC {
            game_state.set_screen(ScreenId::SelectLevelPack);

            return;
        }

        if key == keys::F1 {
            game_state.open_help_page();

            return;
        }

        'outer: {
            match key {
                keys::LEFT => {
                    if self.selected_level == 0 {
                        break 'outer;
                    }

                    self.selected_level -= 1;
                },
                keys::UP => {
                    if self.selected_level < 24 {
                        break 'outer;
                    }

                    self.selected_level -= 24;
                },
                keys::RIGHT => {
                    if self.selected_level + 1 >= game_state.get_current_level_pack().
                            as_ref().unwrap().level_count() {
                        break 'outer;
                    }

                    self.selected_level += 1;
                },
                keys::DOWN => {
                    if self.selected_level + 24 >= game_state.get_current_level_pack().
                            as_ref().unwrap().level_count() {
                        break 'outer;
                    }

                    self.selected_level += 24;
                },

                keys::ENTER if self.selected_level <= game_state.get_current_level_pack().
                        as_ref().unwrap().min_level_not_completed() => {
                    game_state.set_level_index(self.selected_level);
                    game_state.set_screen(ScreenId::InGame);
                },

                _ => {},
            }
        }
    }

    fn on_mouse_pressed(&mut self, game_state: &mut GameState, column: usize, row: usize) {
        if row == 0 {
            return;
        }

        let level_index = column/3 + (row - 1)/2*24;
        if level_index < game_state.get_current_level_pack().as_ref().unwrap().level_count() {
            self.selected_level = level_index;
            self.on_key_pressed(game_state, keys::ENTER);
        }
    }

    fn on_set_screen(&mut self, game_state: &mut GameState) {
        self.selected_level = game_state.get_level_index();
    }
}

pub struct ScreenInGame {
    time_start_in_menu: Option<SystemTime>,
    time_start: Option<SystemTime>,
    time_millis: u32,
    time_sec: u32,
    time_min: u32,

    moves: u32,
    old_moves: u32,

    player_pos: (usize, usize),
    old_player_pos: (usize, usize),

    level_now: Option<Level>,
    level_now_last_step: Option<Level>,

    continue_flag: bool,
    secret_found_flag: bool,
    game_over_flag: bool,
}

impl ScreenInGame {
    pub fn new() -> Self {
        Self {
            time_start_in_menu: Default::default(),
            time_start: Default::default(),
            time_millis: Default::default(),
            time_sec: Default::default(),
            time_min: Default::default(),

            moves: Default::default(),
            old_moves: Default::default(),

            player_pos: Default::default(),
            old_player_pos: Default::default(),

            level_now: Default::default(),
            level_now_last_step: Default::default(),

            continue_flag: Default::default(),
            secret_found_flag: Default::default(),
            game_over_flag: Default::default(),
        }
    }

    pub fn start_level(&mut self, level: &Level) {
        //Reset stats
        self.time_start = None;
        self.time_millis = 0;
        self.time_sec = 0;
        self.time_min = 0;

        self.old_moves = 0;
        self.moves = 0;

        self.level_now = Some(level.clone());
        self.level_now_last_step = Some(level.clone());

        'outer:
        for i in 0..level.width() {
            for j in 0..level.height() {
                if let Some(tile) = level.get_tile(i, j) {
                    if *tile == Tile::Player {
                        self.player_pos = (i, j);
                        self.old_player_pos = (i, j);

                        break 'outer;
                    }
                }
            }
        }
    }

    fn draw_tutorial_level_text(&self, game_state: &GameState, console: &Console) {
        //Draw special help text for tutorial levels (tutorial pack and tutorial levels in special pack)
        if game_state.get_level_pack_index() == 0 { //Tutorial pack
            console.reset_color();
            match game_state.current_level_index {
                0 => {
                    if self.continue_flag {
                        console.set_cursor_pos(18, 8);
                        console.draw_text("Press ");

                        console.set_color(Color::Red, Color::Default);
                        console.draw_text("ENTER");

                        console.reset_color();
                        console.draw_text(" to go to the next level...");
                    }else {
                        console.set_cursor_pos(17, 8);
                        console.draw_text("Use the arrow keys (< ^ > v) to move...");
                    }
                },
                1 => {
                    console.set_cursor_pos(16, 8);
                    console.draw_text("Boxes (");

                    console.set_color(Color::LightCyan, Color::Default);
                    console.draw_text("@");

                    console.reset_color();
                    console.draw_text(") must be placed on ");

                    console.set_color(Color::Red, Color::Default);
                    console.draw_text("all");

                    console.reset_color();
                    console.draw_text(" goals (");

                    console.set_color(Color::Red, Color::Default);
                    console.draw_text("x");

                    console.reset_color();
                    console.draw_text(")");
                },
                2 => {
                    console.set_cursor_pos(14, 8);
                    console.draw_text("Some boxes (");

                    console.set_color(Color::LightPink, Color::Default);
                    console.draw_text("@");

                    console.reset_color();
                    console.draw_text(") might already be in a goal (");

                    console.set_color(Color::Red, Color::Default);
                    console.draw_text("x");

                    console.reset_color();
                    console.draw_text(")");
                },
                3 => {
                    console.set_cursor_pos(14, 8);
                    console.draw_text("Not all boxes (");

                    console.set_color(Color::LightCyan, Color::Default);
                    console.draw_text("@");

                    console.reset_color();
                    console.draw_text(") must be in a goal (");

                    console.set_color(Color::Red, Color::Default);
                    console.draw_text("x");

                    console.reset_color();
                    console.draw_text(") to win");
                },
                4 => {
                    console.set_cursor_pos(5, 8);
                    console.draw_text("One-way doors (");

                    console.set_color(Color::Blue, Color::Default);
                    console.draw_text("< ^ > v");

                    console.reset_color();
                    console.draw_text(") can only be entered from the opened side");
                },
                5 => {
                    if self.game_over_flag {
                        console.set_cursor_pos(12, 8);
                        console.draw_text("Press ");

                        console.set_color(Color::Red, Color::Default);
                        console.draw_text("ESC");

                        console.reset_color();
                        console.draw_text(" to go back to the level selection screen");
                    }else {
                        console.set_cursor_pos(8, 8);
                        console.draw_text("Boxes (");

                        console.set_color(Color::LightCyan, Color::Default);
                        console.draw_text("@");

                        console.reset_color();
                        console.draw_text(") can not be moved through one-way doors (");

                        console.set_color(Color::Blue, Color::Default);
                        console.draw_text("< ^ > v");

                        console.reset_color();
                        console.draw_text(")");
                    }
                },
                _ => {},
            }
        }else if game_state.get_level_pack_index() == 2 { //Built-in special pack
            console.reset_color();
            match game_state.current_level_index {
                0 => {
                    console.set_cursor_pos(18, 8);
                    console.draw_text("Keys (");

                    console.set_color(Color::LightCyan, Color::Default);
                    console.draw_text("*");

                    console.reset_color();
                    console.draw_text(") can be used to open doors (");

                    console.set_color(Color::Red, Color::Default);
                    console.draw_text("=");

                    console.reset_color();
                    console.draw_text(")");
                },
                1 => {
                    console.set_cursor_pos(19, 8);
                    console.draw_text("Every key (");

                    console.set_color(Color::LightCyan, Color::Default);
                    console.draw_text("*");

                    console.reset_color();
                    console.draw_text(") can open any door (");

                    console.set_color(Color::Red, Color::Default);
                    console.draw_text("=");

                    console.reset_color();
                    console.draw_text(")");
                },
                2 => {
                    console.set_cursor_pos(21, 8);
                    console.draw_text("Keys (");

                    console.set_color(Color::LightPink, Color::Default);
                    console.draw_text("*");

                    console.reset_color();
                    console.draw_text(") might be in a goal (");

                    console.set_color(Color::Red, Color::Default);
                    console.draw_text("x");

                    console.reset_color();
                    console.draw_text(")");
                },
                _ => {},
            }
        }
    }
}

impl Screen for ScreenInGame {
    fn draw(&self, game_state: &GameState, console: &Console) {
        console.reset_color();
        console.draw_text(format!("Pack: {:02}", game_state.get_level_pack_index() + 1));

        console.set_cursor_pos(((Game::CONSOLE_MIN_WIDTH - 9) as f64 * 0.25) as usize, 0);
        console.draw_text("Level: ");
        let level = game_state.current_level_index + 1;
        if level < 100 {
            console.draw_text(format!("{:02}", level));
        }else {
            console.draw_text(format!("{}", (b'A' + (level as u8 + 1 - 100) / 10) as char));
            console.draw_text(format!("{}", (level + 1) % 10));
        }

        console.set_cursor_pos(((Game::CONSOLE_MIN_WIDTH - 11) as f64 * 0.75) as usize, 0);
        console.draw_text(format!("Moves: {:04}", self.moves));

        console.set_cursor_pos(Game::CONSOLE_MIN_WIDTH - 15, 0);
        console.draw_text(format!(
            "Time: {:02}:{:02}.{:03}",
            self.time_min,
            self.time_sec,
            self.time_millis,
        ));

        if self.continue_flag {
            console.set_cursor_pos(((Game::CONSOLE_MIN_WIDTH - 16) as f64 * 0.5) as usize, 0);
            console.draw_text("Level completed!");
        }

        if self.game_over_flag {
            if self.secret_found_flag {
                console.set_cursor_pos(((Game::CONSOLE_MIN_WIDTH - 13) as f64 * 0.5) as usize, 0);
                console.draw_text("Secret found!");
            }else {
                console.set_cursor_pos(((Game::CONSOLE_MIN_WIDTH - 13) as f64 * 0.5) as usize, 0);
                console.draw_text("You have won!");
            }
        }

        if let Some(ref level) = self.level_now {
            let x_offset = ((Game::CONSOLE_MIN_WIDTH - level.width()) as f64 * 0.5) as usize;
            let y_offset = 1;

            level.draw(console, x_offset, y_offset, game_state.is_player_background(), None);

            self.draw_tutorial_level_text(game_state, console);
        }
    }

    fn update(&mut self, game_state: &mut GameState) {
        if game_state.is_dialog_opened() || self.game_over_flag || self.continue_flag {
            return;
        }

        if let Some(ref time_start) = self.time_start {
            let time_current = SystemTime::now();

            let diff = time_current.duration_since(*time_start).
                    expect("Time manipulation detected (Start time is in the future)!").
                    as_millis();

            self.time_millis = (diff % 1000) as u32;
            self.time_sec = (diff / 1000 % 60) as u32;
            self.time_min = (diff / 1000 / 60) as u32;
        }
    }

    fn on_key_pressed(&mut self, game_state: &mut GameState, key: i32) {
        if key == keys::ESC {
            if self.game_over_flag {
                self.continue_flag = false;
                self.game_over_flag = false;

                game_state.set_screen(ScreenId::SelectLevel);

                return;
            }

            self.time_start_in_menu = Some(SystemTime::now());

            game_state.open_dialog(DialogYesNo::new("Back to level selection?"));

            return;
        }

        if key == keys::F1 {
            self.time_start_in_menu = Some(SystemTime::now());

            game_state.open_help_page();

            return;
        }

        let current_level_index = game_state.current_level_index;
        let Some(level_pack) = game_state.get_current_level_pack_mut() else {
            return;
        };

        //Level end
        if self.continue_flag {
            if key == keys::ENTER {
                self.continue_flag = false;

                //All levels completed
                if current_level_index + 1 == level_pack.level_count() {
                    self.game_over_flag = true;

                    return;
                }else {
                    game_state.current_level_index += 1;
                }

                self.start_level(game_state.get_current_level_pack().unwrap().levels()[game_state.current_level_index].level());
            }else if key == 'r' as i32 {
                self.continue_flag = false;

                self.start_level(level_pack.levels()[current_level_index].level());
            }

            return;
        }

        //One step back
        if key == 'z' as i32 {
            mem::swap(&mut self.level_now, &mut self.level_now_last_step);

            //Reset move count
            mem::swap(&mut self.moves, &mut self.old_moves);

            //Reset player pos
            mem::swap(&mut self.player_pos, &mut self.old_player_pos);
        }

        //Reset
        if key == 'r' as i32 {
            self.start_level(level_pack.levels()[current_level_index].level());
        }

        if console_lib::is_arrow_key(key) {
            let level_now_before_move = self.level_now.clone();

            let width = self.level_now.as_ref().unwrap().width();
            let height = self.level_now.as_ref().unwrap().height();

            let (x_from, y_from) = self.player_pos;

            let x_to = match key {
                keys::LEFT => if x_from == 0 {
                    width - 1
                }else {
                    x_from - 1
                },
                keys::RIGHT => if x_from == width - 1 {
                    0
                }else {
                    x_from + 1
                },
                _ => x_from,
            };
            let y_to = match key {
                keys::UP => if y_from == 0 {
                    height - 1
                }else {
                    y_from - 1
                },
                keys::DOWN => if y_from == height - 1 {
                    0
                }else {
                    y_from + 1
                },
                _ => y_from,
            };

            let one_way_door_tile = match key {
                keys::LEFT => Tile::OneWayLeft,
                keys::UP => Tile::OneWayUp,
                keys::RIGHT => Tile::OneWayRight,
                keys::DOWN => Tile::OneWayDown,
                _ => return, //Should never happen
            };

            //Set players old position to old level data
            let mut tile = level_pack.levels()[current_level_index].level().get_tile(x_from, y_from).unwrap().clone();
            if tile == Tile::Player || tile == Tile::Box || tile == Tile::Key || tile == Tile::LockedDoor {
                tile = Tile::Empty;
            }else if tile == Tile::BoxInGoal || tile == Tile::KeyInGoal {
                tile = Tile::Goal;
            }

            self.level_now.as_mut().unwrap().set_tile(x_from, y_from, tile);

            self.time_start.get_or_insert_with(SystemTime::now);

            let mut has_won = false;
            let tile = self.level_now.as_ref().unwrap().get_tile(x_to, y_to).unwrap().clone();
            if matches!(tile, Tile::Empty | Tile::Goal | Tile::Secret) || tile == one_way_door_tile {
                if tile == Tile::Secret {
                    self.game_over_flag = true;
                    self.secret_found_flag = true;
                }

                self.player_pos = (x_to, y_to);
            }else if matches!(tile, Tile::Box | Tile::BoxInGoal | Tile::Key | Tile::KeyInGoal if self.level_now.as_mut().unwrap().move_box_or_key(
                level_pack.levels().get(current_level_index).unwrap().level(),
                &mut has_won, x_from, y_from, x_to, y_to
            )) {
                self.player_pos = (x_to, y_to);
            }

            //Set player to new position
            self.level_now.as_mut().unwrap().set_tile(self.player_pos.0, self.player_pos.1, Tile::Player);

            //Copy level to last step if change
            if self.player_pos != (x_from, y_from) {
                self.old_moves = self.moves;
                self.moves += 1;

                self.old_player_pos = (x_from, y_from);
                self.level_now_last_step = level_now_before_move;
            }

            if has_won {
                self.continue_flag = true;

                //Update best scores
                let time = self.time_millis as u64 + 1000 * self.time_sec as u64 + 60000 * self.time_min as u64;
                let moves = self.moves;

                level_pack.update_stats(current_level_index, time, moves);

                if current_level_index >= level_pack.min_level_not_completed() {
                    level_pack.set_min_level_not_completed(current_level_index + 1);
                }

                if let Err(err) = level_pack.save_save_game() {
                    game_state.open_dialog(DialogOk::new_error(format!("Can not save: {}", err)));
                }
            }

            if self.secret_found_flag {
                game_state.open_dialog(DialogOk::new("You have found a secret!"));

                if let Err(err) = game_state.on_found_secret() {
                    game_state.open_dialog(DialogOk::new_error(format!("Error: {}", err)));
                }
            }
        }
    }

    fn on_dialog_selection(&mut self, game_state: &mut GameState, selection: DialogSelection) {
        if self.secret_found_flag {
            self.continue_flag = false;
            self.game_over_flag = false;
            self.secret_found_flag = false;

            game_state.set_screen(ScreenId::SelectLevelPack);

            return;
        }

        if selection == DialogSelection::Yes {
            self.continue_flag = false;
            self.game_over_flag = false;

            game_state.set_screen(ScreenId::SelectLevel);
        }else if selection == DialogSelection::No {
            self.on_continue(game_state);
        }
    }

    fn on_continue(&mut self, _: &mut GameState) {
        if self.game_over_flag || self.continue_flag || self.time_start.is_none() || self.time_start_in_menu.is_none() {
            return;
        }

        let diff = SystemTime::now().duration_since(self.time_start_in_menu.take().unwrap()).
                expect("Time manipulation detected (Start time is in the future)!");

        self.time_start = self.time_start.map(|time_start| time_start + diff);
    }

    fn on_set_screen(&mut self, game_state: &mut GameState) {
        self.start_level(game_state.get_current_level_pack().as_ref().unwrap().levels().get(
            game_state.get_level_index()).unwrap().level());
    }
}

pub struct ScreenSelectLevelPackEditor {
    is_creating_new_level_pack: bool,
    new_level_pack_id: String,
}

impl ScreenSelectLevelPackEditor {
    pub fn new() -> Self {
        Self {
            is_creating_new_level_pack: Default::default(),
            new_level_pack_id: String::new(),
        }
    }
}

impl Screen for ScreenSelectLevelPackEditor {
    fn draw(&self, game_state: &GameState, console: &Console) {
        console.reset_color();
        console.set_underline(true);
        console.draw_text("Edit a level pack:");
        console.set_underline(false);

        //Include Create Level Pack entry
        let entry_count = game_state.editor_state.get_level_pack_count() + 1;

        //Draw first line
        console.set_cursor_pos(0, 1);
        console.draw_text("-");
        let mut max = entry_count%24;
        if entry_count/24 > 0 {
            max = 24;
        }

        for i in 0..max  {
            let x = 1 + (i%24)*3;

            console.set_cursor_pos(x, 1);
            console.draw_text("---");
        }

        for i in 0..entry_count {
            let x = 1 + (i%24)*3;
            let y = 2 + (i/24)*2;

            //First box
            if x == 1 {
                console.set_cursor_pos(x - 1, y);
                console.draw_text("|");

                console.set_cursor_pos(x - 1, y + 1);
                console.draw_text("-");
            }

            console.set_cursor_pos(x, y);
            if i == game_state.editor_state.get_level_pack_count() {
                //Level Pack Editor entry
                console.set_color(Color::White, Color::LightBlue);
                console.draw_text(" +");
            }else {
                console.set_color(Color::Black, Color::Green);
                console.draw_text(format!("{:2}", i + 1));
            }

            console.reset_color();
            console.draw_text("|");

            console.set_cursor_pos(x, y + 1);
            console.draw_text("---");
        }

        //Mark selected level
        let x = (game_state.editor_state.get_level_pack_index()%24)*3;
        let y = 1 + (game_state.editor_state.get_level_pack_index()/24)*2;

        console.set_color(Color::Cyan, Color::Default);
        console.set_cursor_pos(x, y);
        console.draw_text("----");
        console.set_cursor_pos(x, y + 1);
        console.draw_text("|");
        console.set_cursor_pos(x + 3, y + 1);
        console.draw_text("|");
        console.set_cursor_pos(x, y + 2);
        console.draw_text("----");

        //Draw border for best time and best moves
        let y = 4 + (entry_count/24)*2;

        console.set_cursor_pos(0, y);
        console.set_color(Color::Cyan, Color::Default);
        console.draw_text(".------------------------------------------------------------------------.");
        for i in 1..4 {
            console.set_cursor_pos(0, y + i);
            console.draw_text("|                                                                        |");
        }
        console.set_cursor_pos(0, y + 4);
        console.draw_text("\'------------------------------------------------------------------------\'");
        console.reset_color();

        if self.is_creating_new_level_pack {
            console.set_cursor_pos(1, y + 1);
            console.draw_text("Enter a new level pack ID:");

            console.set_cursor_pos(1, y + 2);
            console.draw_text(format!("> {}", &self.new_level_pack_id));
        }else if game_state.editor_state.get_level_pack_index() == game_state.editor_state.get_level_pack_count() {
            //Level Pack Editor entry
            console.set_cursor_pos(29, y + 2);
            console.draw_text("Create level pack");
        }else {
            console.set_cursor_pos(1, y + 1);
            console.draw_text(format!("Level Pack ID: {}", game_state.editor_state.get_current_level_pack().unwrap().id()));

            console.set_cursor_pos(1, y + 2);
            console.draw_text(format!("Levels: {}", game_state.editor_state.get_current_level_pack().unwrap().level_count()));
        }
    }

    fn on_key_pressed(&mut self, game_state: &mut GameState, key: i32) {
        if self.is_creating_new_level_pack {
            match key {
                key if (0..=127).contains(&key) &&
                        ((key as u8 as char).is_alphanumeric() || key as u8 == b'_' || key as u8 == b'-') => {
                    if self.new_level_pack_id.len() >= Game::MAX_LEVEL_PACK_ID_LEN {
                        return;
                    }
                    
                    self.new_level_pack_id += &format!("{}", key as u8 as char);
                },
                keys::DELETE => {
                    self.new_level_pack_id.pop();
                },

                keys::ENTER => {
                    if self.new_level_pack_id.len() < 3 {
                        game_state.open_dialog(DialogOk::new_error("Level pack ID must have at least 3 characters!"));

                        return;
                    }

                    //TODO check if does not already exist and open level pack editor

                    let Ok(mut save_game_file) = Game::get_or_create_save_game_folder() else {
                        game_state.open_dialog(DialogOk::new_error("Can not save!"));

                        return;
                    };
                    save_game_file.push(&self.new_level_pack_id);
                    save_game_file.push(".lvl.edit");

                    let Some(save_game_file) = save_game_file.to_str() else {
                        game_state.open_dialog(DialogOk::new_error("Can not save!"));

                        return;
                    };

                    let level_pack = LevelPack::new(&self.new_level_pack_id, save_game_file);
                    if let Err(err) = level_pack.save_editor_level_pack() {
                        game_state.open_dialog(DialogOk::new_error(format!("Can not save: {}", err)));
                    }

                    game_state.editor_state.level_packs.push(level_pack);
                    game_state.editor_state.set_level_index(0);

                    self.is_creating_new_level_pack = false;
                    self.new_level_pack_id = String::new();

                    game_state.editor_state.set_level_index(0);
                    game_state.set_screen(ScreenId::LevelPackEditor);
                },

                keys::ESC => {
                    self.is_creating_new_level_pack = false;
                    self.new_level_pack_id = String::new();
                },

                _ => {},
            }

            return;
        }

        if key == keys::ESC {
            game_state.set_screen(ScreenId::SelectLevelPack);

            return;
        }

        if key == keys::F1 {
            game_state.open_help_page();

            return;
        }

        //TODO handle enter

        'outer: {
            //Include Level Pack Editor entry
            let entry_count = game_state.editor_state.get_level_pack_count() + 1;

            match key {
                keys::LEFT => {
                    if game_state.editor_state.selected_level_pack_index == 0 {
                        break 'outer;
                    }

                    game_state.editor_state.selected_level_pack_index -= 1;
                },
                keys::UP => {
                    if game_state.editor_state.selected_level_pack_index <= 24 {
                        break 'outer;
                    }

                    game_state.editor_state.selected_level_pack_index -= 24;
                },
                keys::RIGHT => {
                    if game_state.editor_state.selected_level_pack_index + 1 >= entry_count {
                        break 'outer;
                    }

                    game_state.editor_state.selected_level_pack_index += 1;
                },
                keys::DOWN => {
                    if game_state.editor_state.selected_level_pack_index + 24 >= entry_count {
                        break 'outer;
                    }

                    game_state.editor_state.selected_level_pack_index += 24;
                },

                keys::ENTER => {
                    if game_state.editor_state.selected_level_pack_index == game_state.editor_state.get_level_pack_count() {
                        //Level Pack Editor entry
                        self.is_creating_new_level_pack = true;
                    }else {
                        //Set selected level pack
                        game_state.editor_state.set_level_index(0);
                        game_state.set_screen(ScreenId::LevelPackEditor);
                    }
                },

                _ => {},
            }
        }
    }

    fn on_mouse_pressed(&mut self, game_state: &mut GameState, column: usize, row: usize) {
        if row == 0 {
            return;
        }
        //Include Level Pack Editor entry
        let entry_count = game_state.editor_state.get_level_pack_count() + 1;

        let level_pack_index = column/3 + (row - 1)/2*24;
        if level_pack_index < entry_count {
            game_state.editor_state.selected_level_pack_index = level_pack_index;
            self.on_key_pressed(game_state, keys::ENTER);
        }
    }
}

pub struct ScreenLevelPackEditor {
    is_creating_new_level: bool,
    is_editing_height: bool,
    is_deleting_level: bool,
    new_level_width_str: String,
    new_level_height_str: String,
}

impl ScreenLevelPackEditor {
    pub fn new() -> Self {
        Self {
            is_creating_new_level: Default::default(),
            is_editing_height: Default::default(),
            is_deleting_level: Default::default(),
            new_level_width_str: String::new(),
            new_level_height_str: String::new(),
        }
    }
}

impl Screen for ScreenLevelPackEditor {
    fn draw(&self, game_state: &GameState, console: &Console) {
        console.reset_color();
        console.set_underline(true);
        console.draw_text(format!("Edit a level (Level pack \"{}\"):", game_state.editor_state.get_current_level_pack().unwrap().id()));
        console.set_underline(false);

        //Include Create Level entry
        let entry_count = game_state.editor_state.get_current_level_pack().unwrap().level_count() + 1;

        //Draw first line
        console.set_cursor_pos(0, 1);
        console.draw_text("-");
        let mut max = entry_count%24;
        if entry_count/24 > 0 {
            max = 24;
        }

        for i in 0..max  {
            let x = 1 + (i%24)*3;

            console.set_cursor_pos(x, 1);
            console.draw_text("---");
        }

        for i in 0..entry_count {
            let x = 1 + (i%24)*3;
            let y = 2 + (i/24)*2;

            //First box
            if x == 1 {
                console.set_cursor_pos(x - 1, y);
                console.draw_text("|");

                console.set_cursor_pos(x - 1, y + 1);
                console.draw_text("-");
            }

            console.set_cursor_pos(x, y);
            if i == game_state.editor_state.get_current_level_pack().unwrap().level_count() {
                //Level Pack entry
                console.set_color(Color::White, Color::LightBlue);
                console.draw_text(" +");
            }else {
                console.set_color(Color::Black, Color::Green);
                if i + 1 < 100 {
                    console.draw_text(format!("{:2}", i + 1));
                }else {
                    console.draw_text(format!("{}", (b'A' + (i as u8 + 1 - 100) / 10) as char));
                    console.draw_text(format!("{}", (i + 1) % 10));
                }
            }

            console.reset_color();
            console.draw_text("|");

            console.set_cursor_pos(x, y + 1);
            console.draw_text("---");
        }

        //Mark selected level
        let x = (game_state.editor_state.get_level_index()%24)*3;
        let y = 1 + (game_state.editor_state.get_level_index()/24)*2;

        console.set_color(Color::Cyan, Color::Default);
        console.set_cursor_pos(x, y);
        console.draw_text("----");
        console.set_cursor_pos(x, y + 1);
        console.draw_text("|");
        console.set_cursor_pos(x + 3, y + 1);
        console.draw_text("|");
        console.set_cursor_pos(x, y + 2);
        console.draw_text("----");

        //Draw border for best time and best moves
        let y = 4 + (entry_count/24)*2;

        console.set_cursor_pos(0, y);
        console.set_color(Color::Cyan, Color::Default);
        console.draw_text(".------------------------------------------------------------------------.");
        for i in 1..4 {
            console.set_cursor_pos(0, y + i);
            console.draw_text("|                                                                        |");
        }
        console.set_cursor_pos(0, y + 4);
        console.draw_text("\'------------------------------------------------------------------------\'");
        console.reset_color();

        if self.is_creating_new_level {
            console.set_cursor_pos(1, y + 1);
            console.draw_text("Enter width and height for new level:");

            console.set_color(if self.is_editing_height {
                Color::LightBlue
            }else {
                Color::Cyan
            }, Color::Default);
            console.set_cursor_pos(1, y + 2);
            console.draw_text(format!("Width: {}", &self.new_level_width_str));

            console.set_color(if self.is_editing_height {
                Color::Cyan
            }else {
                Color::LightBlue
            }, Color::Default);
            console.set_cursor_pos(14, y + 2);
            console.draw_text(format!("Height: {}", &self.new_level_height_str));
        }else if game_state.editor_state.get_level_index() == game_state.editor_state.get_current_level_pack().unwrap().level_count() {
            //Level Pack Editor entry
            console.set_cursor_pos(29, y + 2);
            console.draw_text("Create level");
        }else {
            //Draw best time and best moves
            console.set_cursor_pos(1, y + 1);
            console.draw_text("Selected level: ");
            let selected_level = game_state.editor_state.selected_level_index;
            if selected_level + 1 < 100 {
                console.draw_text(format!("{:02}", selected_level + 1));
            }else {
                console.draw_text(format!("{}", (b'A' + (selected_level as u8 + 1 - 100) / 10) as char));
                console.draw_text(format!("{}", (selected_level + 1) % 10));
            }

            console.set_cursor_pos(1, y + 2);
            console.draw_text(format!(
                "Size: {} x {}",
                game_state.editor_state.get_current_level().unwrap().width(),
                game_state.editor_state.get_current_level().unwrap().height(),
            ));
        }
    }

    fn on_key_pressed(&mut self, game_state: &mut GameState, key: i32) {
        if self.is_creating_new_level {
            match key {
                key if (0..=127).contains(&key) && ((key as u8 as char).is_numeric()) => {
                    if self.is_editing_height {
                        if self.new_level_height_str.len() >= 2 {
                            return;
                        }

                        self.new_level_height_str += &format!("{}", key as u8 as char);
                    }else {
                        if self.new_level_width_str.len() >= 2 {
                            return;
                        }

                        self.new_level_width_str += &format!("{}", key as u8 as char);
                    }
                },
                keys::DELETE => {
                    if self.is_editing_height {
                        self.new_level_height_str.pop();
                    }else {
                        self.new_level_width_str.pop();
                    }
                },

                keys::TAB => {
                    self.is_editing_height = !self.is_editing_height;
                },

                keys::ENTER => {
                    if !(1..=2).contains(&self.new_level_width_str.len()) {
                        game_state.open_dialog(DialogOk::new_error(format!("Width must be >= 3 and <= {}!", Game::CONSOLE_MIN_WIDTH)));

                        return;
                    }

                    let Ok(width) = usize::from_str(&self.new_level_width_str) else {
                        game_state.open_dialog(DialogOk::new_error("Width must be a number"));

                        return;
                    };

                    if !(3..=Game::CONSOLE_MIN_WIDTH).contains(&width) {
                        game_state.open_dialog(DialogOk::new_error(format!("Width must be >= 3 and <= {}!", Game::CONSOLE_MIN_WIDTH)));

                        return;
                    }

                    if !(1..=2).contains(&self.new_level_height_str.len()) {
                        game_state.open_dialog(DialogOk::new_error(format!("Height must be >= 3 and <= {}!", Game::CONSOLE_MIN_WIDTH)));

                        return;
                    }

                    let Ok(height) = usize::from_str(&self.new_level_height_str) else {
                        game_state.open_dialog(DialogOk::new_error("Height must be a number"));

                        return;
                    };

                    if !(3..Game::CONSOLE_MIN_HEIGHT).contains(&height) {
                        game_state.open_dialog(DialogOk::new_error(format!("Height must be >= 3 and <= {}!", Game::CONSOLE_MIN_WIDTH)));

                        return;
                    }

                    game_state.editor_state.get_current_level_pack_mut().unwrap().add_level(Level::new(width, height));

                    //TODO save

                    self.is_creating_new_level = false;
                    self.is_editing_height = false;
                    self.is_deleting_level = false;
                    self.new_level_width_str = String::new();
                    self.new_level_height_str = String::new();

                    game_state.set_screen(ScreenId::LevelEditor);
                },

                keys::ESC => {
                    self.is_creating_new_level = false;
                    self.is_editing_height = false;
                    self.is_deleting_level = false;
                    self.new_level_width_str = String::new();
                    self.new_level_height_str = String::new();
                },

                _ => {},
            }

            return;
        }

        if key == keys::ESC {
            game_state.set_screen(ScreenId::SelectLevelPackEditor);

            return;
        }

        if key == keys::F1 {
            game_state.open_help_page();

            return;
        }

        //TODO handle enter

        'outer: {
            //Include Level Pack Editor entry
            let entry_count = game_state.editor_state.get_current_level_pack().unwrap().level_count() + 1;

            match key {
                keys::LEFT => {
                    if game_state.editor_state.selected_level_index == 0 {
                        break 'outer;
                    }

                    game_state.editor_state.selected_level_index -= 1;
                },
                keys::UP => {
                    if game_state.editor_state.selected_level_index <= 24 {
                        break 'outer;
                    }

                    game_state.editor_state.selected_level_index -= 24;
                },
                keys::RIGHT => {
                    if game_state.editor_state.selected_level_index + 1 >= entry_count {
                        break 'outer;
                    }

                    game_state.editor_state.selected_level_index += 1;
                },
                keys::DOWN => {
                    if game_state.editor_state.selected_level_index + 24 >= entry_count {
                        break 'outer;
                    }

                    game_state.editor_state.selected_level_index += 24;
                },

                keys::ENTER => {
                    if game_state.editor_state.selected_level_index == game_state.editor_state.get_current_level_pack().unwrap().level_count() {
                        //Level Pack entry
                        self.is_creating_new_level = true;
                    }else {
                        //Set selected level
                        game_state.set_screen(ScreenId::LevelEditor);
                    }
                },

                keys::DELETE => {
                    if game_state.editor_state.selected_level_index != game_state.editor_state.get_current_level_pack().unwrap().level_count() {
                        self.is_deleting_level = true;

                        game_state.open_dialog(DialogYesNo::new(format!("Do you really want to delete level {}?", game_state.editor_state.selected_level_index + 1)));
                    }
                },

                _ => {},
            }
        }
    }

    fn on_mouse_pressed(&mut self, game_state: &mut GameState, column: usize, row: usize) {
        if row == 0 {
            return;
        }

        //Include create Level entry
        let entry_count = game_state.editor_state.get_current_level_pack().unwrap().level_count() + 1;

        let level_pack_index = column/3 + (row - 1)/2*24;
        if level_pack_index < entry_count {
            game_state.editor_state.selected_level_index = level_pack_index;
            self.on_key_pressed(game_state, keys::ENTER);
        }
    }

    fn on_dialog_selection(&mut self, game_state: &mut GameState, selection: DialogSelection) {
        if self.is_deleting_level {
            if selection == DialogSelection::Yes {
                let index = game_state.editor_state.selected_level_index;
                game_state.editor_state.get_current_level_pack_mut().unwrap().levels_mut().remove(index);
                if let Err(err) = game_state.editor_state.get_current_level_pack().unwrap().save_editor_level_pack() {
                    game_state.open_dialog(DialogOk::new_error(format!("Can not save: {}", err)));
                }

                self.is_deleting_level = false;
            }else if selection == DialogSelection::No {
                self.is_deleting_level = false;
            }
        }
    }
}

pub struct ScreenLevelEditor {
    level: Option<Level>,
    is_vertical_input: bool,
    is_reverse_input: bool,
    playing_level: Option<Level>,
    cursor_pos: (usize, usize),
    player_pos: (usize, usize),
}

impl ScreenLevelEditor {
    pub fn new() -> Self {
        Self {
            level: Default::default(),
            is_vertical_input: Default::default(),
            is_reverse_input: Default::default(),
            playing_level: Default::default(),
            cursor_pos: Default::default(),
            player_pos: Default::default(),
        }
    }

    fn on_key_pressed_playing(&mut self, key: i32) {
        if let Some(level) = self.playing_level.as_mut() {
            if console_lib::is_arrow_key(key) {
                let width = level.width();
                let height = level.height();

                let (x_from, y_from) = self.player_pos;

                let x_to = match key {
                    keys::LEFT => if x_from == 0 {
                        width - 1
                    }else {
                        x_from - 1
                    },
                    keys::RIGHT => if x_from == width - 1 {
                        0
                    }else {
                        x_from + 1
                    },
                    _ => x_from,
                };
                let y_to = match key {
                    keys::UP => if y_from == 0 {
                        height - 1
                    }else {
                        y_from - 1
                    },
                    keys::DOWN => if y_from == height - 1 {
                        0
                    }else {
                        y_from + 1
                    },
                    _ => y_from,
                };

                let one_way_door_tile = match key {
                    keys::LEFT => Tile::OneWayLeft,
                    keys::UP => Tile::OneWayUp,
                    keys::RIGHT => Tile::OneWayRight,
                    keys::DOWN => Tile::OneWayDown,
                    _ => return, //Should never happen
                };

                //Set players old position to old level data
                let mut tile = self.level.as_ref().unwrap().get_tile(x_from, y_from).unwrap().clone();
                if tile == Tile::Player || tile == Tile::Box || tile == Tile::Key || tile == Tile::LockedDoor {
                    tile = Tile::Empty;
                }else if tile == Tile::BoxInGoal || tile == Tile::KeyInGoal {
                    tile = Tile::Goal;
                }

                level.set_tile(x_from, y_from, tile);

                let mut has_won = false;
                let tile = level.get_tile(x_to, y_to).unwrap().clone();
                if matches!(tile, Tile::Empty | Tile::Goal | Tile::Secret) || tile == one_way_door_tile {
                    self.player_pos = (x_to, y_to);
                }else if matches!(tile, Tile::Box | Tile::BoxInGoal | Tile::Key | Tile::KeyInGoal if level.move_box_or_key(
                    self.level.as_ref().unwrap(),
                    &mut has_won, x_from, y_from, x_to, y_to
                )) {
                    self.player_pos = (x_to, y_to);
                }

                //Set player to new position
                level.set_tile(self.player_pos.0, self.player_pos.1, Tile::Player);

                //Copy level to last step if change
                if self.player_pos != (x_from, y_from) {
                    //TODO moves += 1
                }
            }
        }
    }

    fn on_key_pressed_editing(&mut self, key: i32) {
        match key {
            keys::LEFT => {
                if self.cursor_pos.0 > 0 {
                    self.cursor_pos.0 -= 1;
                }else if let Some(ref level) = self.level {
                    self.cursor_pos.0 = level.width() - 1;
                }
            },
            keys::UP => {
                if self.cursor_pos.1 > 0 {
                    self.cursor_pos.1 -= 1;
                }else if let Some(ref level) = self.level {
                    self.cursor_pos.1 = level.height() - 1;
                }
            },
            keys::RIGHT => {
                if self.level.as_ref().is_some_and(|level| self.cursor_pos.0 < level.width() - 1) {
                    self.cursor_pos.0 += 1;
                }else {
                    self.cursor_pos.0 = 0;
                }
            },
            keys::DOWN => {
                if self.level.as_ref().is_some_and(|level| self.cursor_pos.1 < level.height() - 1) {
                    self.cursor_pos.1 += 1;
                }else {
                    self.cursor_pos.1 = 0;
                }
            },

            _ => {},
        }


        if (0..=127).contains(&key) {
            let tile = self.level.as_mut().unwrap().get_tile_mut(self.cursor_pos.0, self.cursor_pos.1).unwrap();

            match key as u8 {
                key @ (b'd' | b'w' | b'a' | b's') => {
                    self.is_vertical_input = key == b'w' || key == b's';
                    self.is_reverse_input = key == b'w' || key == b'a';

                    return;
                },

                key => {
                    if let Ok(tile_input) = Tile::from_ascii(key) {
                        if tile_input != Tile::Secret {
                            *tile = tile_input;
                        }
                    }
                }
            }

            if self.is_vertical_input {
                self.on_key_pressed_editing(if self.is_reverse_input {
                    keys::UP
                }else {
                    keys::DOWN
                });
            }else {
                self.on_key_pressed_editing(if self.is_reverse_input {
                    keys::LEFT
                }else {
                    keys::RIGHT
                });
            }
        }
    }
}

impl Screen for ScreenLevelEditor {
    fn draw(&self, game_state: &GameState, console: &Console) {
        console.reset_color();
        if self.playing_level.is_some() {
            console.draw_text("Playing");

            //TODO (Moves, time, ...)
        }else {
            console.draw_text(format!(
                "Editing ({})",
                match self.is_vertical_input {
                    true if self.is_reverse_input => "^",
                    true => "v",
                    false if self.is_reverse_input => "<",
                    false => ">",
                }
            ));

            console.set_cursor_pos(((Game::CONSOLE_MIN_WIDTH - 14) as f64 * 0.5) as usize, 0);
            console.draw_text(format!("Cursor ({:02}:{:02})", self.cursor_pos.0 + 1, self.cursor_pos.1 + 1));
        }

        if let Some(ref level) = self.level {
            let x_offset = ((Game::CONSOLE_MIN_WIDTH - level.width()) as f64 * 0.5) as usize;
            let y_offset = 1;

            self.playing_level.as_ref().map_or(level, |level| level).
                    draw(console, x_offset, y_offset, game_state.is_player_background(),
                         self.playing_level.as_ref().map_or(Some(self.cursor_pos), |_| None));
        }
    }

    fn on_key_pressed(&mut self, game_state: &mut GameState, key: i32) {
        if key == keys::ESC {
            //TODO use dialog with cancel option
            game_state.open_dialog(DialogYesNo::new("Exiting (Save changed?)"));

            return;
        }

        if key == b'r' as i32 {
            self.playing_level = if self.playing_level.is_some() {
                None
            }else {
                'outer:
                for i in 0..self.level.as_ref().unwrap().width() {
                    for j in 0..self.level.as_ref().unwrap().height() {
                        if let Some(tile) = self.level.as_ref().unwrap().get_tile(i, j) {
                            if *tile == Tile::Player {
                                self.player_pos = (i, j);

                                break 'outer;
                            }
                        }
                    }
                }

                //TODO error if no player found
                //TODO error if more than one player found

                self.level.clone()
            };

            return;
        }

        if self.playing_level.is_none() {
            self.on_key_pressed_editing(key);
        }else {
            self.on_key_pressed_playing(key);
        }
    }

    fn on_mouse_pressed(&mut self, _: &mut GameState, column: usize, row: usize) {
        if row == 0 || self.playing_level.is_some() {
            return;
        }

        if let Some(ref level) = self.level {
            let x_offset = ((Game::CONSOLE_MIN_WIDTH - level.width()) as f64 * 0.5) as usize;
            let y_offset = 1;

            if column < x_offset {
                return;
            }

            let x = column - x_offset;
            if x >= level.width() {
                return;
            }

            let y = row - y_offset;
            if y >= level.height() {
                return;
            }

            self.cursor_pos = (x, y);
        }
    }

    fn on_dialog_selection(&mut self, game_state: &mut GameState, selection: DialogSelection) {
        if selection == DialogSelection::Yes {
            *game_state.editor_state.get_current_level_mut().unwrap() = self.level.take().unwrap();
            if let Err(err) = game_state.editor_state.get_current_level_pack().unwrap().save_editor_level_pack() {
                game_state.open_dialog(DialogOk::new_error(format!("Can not save: {}", err)));
            }

            game_state.set_screen(ScreenId::LevelPackEditor);
        }else if selection == DialogSelection::No {
            self.level = None;

            game_state.set_screen(ScreenId::LevelPackEditor);
        }
    }

    fn on_set_screen(&mut self, game_state: &mut GameState) {
        self.is_vertical_input = false;
        self.is_reverse_input = false;
        self.playing_level = None;
        self.cursor_pos = (0, 0);
        self.player_pos = (0, 0);

        self.level = Some(game_state.editor_state.get_current_level().unwrap().clone());
    }
}

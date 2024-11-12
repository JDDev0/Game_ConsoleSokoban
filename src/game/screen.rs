use console_lib::{keys, Color, Console};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::time::SystemTime;
use crate::game::Game;
use crate::game::level::{Level, Tile};

pub struct Dialog {
    message: String
}

impl Dialog {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }

    pub fn draw(&self, console: &Console, console_width: usize, console_height: usize) {
        let char_count = self.message.chars().count();

        let width = char_count.max(16);
        let width_with_border = width + 2;

        let x_start = ((console_width - width_with_border) as f64 * 0.5) as usize;
        let y_start = ((console_height - 6) as f64 * 0.5) as usize;

        let whitespace_count_half = ((width - char_count) as f64 * 0.5) as usize;

        console.set_color(Color::Black, Color::Yellow);
        console.set_cursor_pos(x_start + 1, y_start + 1);
        console.draw_text(format!(
            "{}{}{}",
            " ".repeat(whitespace_count_half),
            self.message,
            " ".repeat(width - char_count - whitespace_count_half),
        ));

        console.set_cursor_pos(x_start + 1, y_start + 2);
        console.draw_text(format!(
            "{}{}{}",
            " ".repeat(whitespace_count_half),
            "-".repeat(char_count),
            " ".repeat(width - char_count - whitespace_count_half),
        ));

        console.set_cursor_pos(x_start + 1, y_start + 3);
        console.draw_text(" ".repeat(width));

        console.set_cursor_pos(x_start + 1, y_start + 4);
        console.draw_text(format!(
            "[y]es{}[n]o",
            " ".repeat(width - 9),
        ));

        //Draw border
        console.set_color(Color::LightBlack, Color::Red);
        console.set_cursor_pos(x_start, y_start);
        console.draw_text(" ".repeat(width_with_border));

        console.set_cursor_pos(x_start, y_start + 5);
        console.draw_text(" ".repeat(width_with_border));
        for i in y_start+1..y_start+5 {
            console.set_cursor_pos(x_start, i);
            console.draw_text(" ");

            console.set_cursor_pos(x_start + width_with_border - 1, i);
            console.draw_text(" ");
        }
    }

    ///Returns `Some(true)` if `[y]es` was pressed and `Some(false)` if `[n]o` was pressed else `None`
    pub fn on_mouse_pressed(&self, console_width: usize, console_height: usize, column: usize, row: usize) -> Option<bool> {
        let char_count = self.message.chars().count();

        let width = char_count.max(16);
        let width_with_border = width + 2;

        let x_start = ((console_width - width_with_border) as f64 * 0.5) as usize;
        let y_start = ((console_height - 6) as f64 * 0.5) as usize;

        if row == y_start + 4 {
            if (x_start + 1..x_start + 6).contains(&column) {
                return Some(true);
            }else if (x_start + width - 3..x_start + width + 1).contains(&column) {
                return Some(false);
            }
        }

        None
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ScreenId {
    StartMenu,
    SelectLevelPack,
    SelectLevel,
    InGame,
}

#[allow(unused_variables)]
pub trait Screen {
    fn draw(&self, game: &Game, console: &Console);

    fn update(&self, game: &Game) {}

    fn on_key_pressed(&self, game: &Game, key: i32) {}
    fn on_mouse_pressed(&self, game: &Game, column: usize, row: usize) {}

    fn on_continue(&self, game: &Game) {}
    fn on_set_screen(&self, game: &Game) {}
}

pub struct ScreenStartMenu {}

impl ScreenStartMenu {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for ScreenStartMenu {
    fn draw(&self, _: &Game, console: &Console) {//Draw border (top)
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

    fn on_key_pressed(&self, game: &Game, key: i32) {
        if game.is_dialog_opened() {
            if key == b'y' as i32 {
                game.close_dialog();
                game.exit();

                return;
            }else if key == b'n' as i32 {
                game.close_dialog();
            }

            return;
        }

        if key == keys::ESC {
            game.open_dialog(Dialog::new("Exit game?"));

            return;
        }

        if key == keys::F1 {
            game.open_help_page();

            return;
        }

        if key == keys::ENTER {
            game.set_screen(ScreenId::SelectLevelPack);
        }
    }

    fn on_mouse_pressed(&self, game: &Game, column: usize, row: usize) {
        if game.is_dialog_opened() {
            return;
        }

        if row == 16 && column > 26 && column < 32 {
            self.on_key_pressed(game, keys::ENTER);
        }

        if row == 21 && column > 64 && column < 73 {
            game.open_help_page();
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
    fn draw(&self, game: &Game, console: &Console) {
        console.reset_color();
        console.set_underline(true);
        console.draw_text("Select a level pack:");
        console.set_underline(false);

        //Draw first line
        console.set_cursor_pos(0, 1);
        console.draw_text("-");
        let mut max = game.get_level_pack_count()%24;
        if game.get_level_pack_count()/24 > 0 {
            max = 24;
        }

        for i in 0..max  {
            let x = 1 + (i%24)*3;

            console.set_cursor_pos(x, 1);
            console.draw_text("---");
        }

        for i in 0..game.get_level_pack_count() {
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
            console.set_color(Color::Black, if game.level_packs().borrow().get(i).
                    unwrap().level_pack_best_moves_sum().is_some() {
                Color::Green
            }else {
                Color::Yellow
            });
            console.draw_text(format!("{:2}", i + 1));

            console.reset_color();
            console.draw_text("|");

            console.set_cursor_pos(x, y + 1);
            console.draw_text("---");
        }

        //Mark selected level
        let x = (game.get_level_pack_index()%24)*3;
        let y = 1 + (game.get_level_pack_index()/24)*2;

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
        let y = 4 + (game.get_level_pack_count()/24)*2;

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
        //Draw sum of best time and sum of best moves
        console.set_cursor_pos(1, y + 1);
        console.draw_text(format!("Selected level pack:             {:02}", game.get_level_pack_index() + 1));
        console.set_cursor_pos(1, y + 2);
        console.draw_text("Sum of best time   : ");
        match game.get_current_level_pack().as_ref().unwrap().level_pack_best_time_sum() {
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
        match game.get_current_level_pack().as_ref().unwrap().level_pack_best_moves_sum() {
            None => console.draw_text("XXXXXXX"),
            Some(best_moves_sum) => console.draw_text(format!("{:07}", best_moves_sum)),
        }
    }

    fn on_key_pressed(&self, game: &Game, key: i32) {
        if key == keys::ESC {
            game.set_screen(ScreenId::StartMenu);

            return;
        }

        if key == keys::F1 {
            game.open_help_page();

            return;
        }

        'outer: {
            match key {
                keys::LEFT => {
                    if *game.current_level_pack_index.borrow() == 0 {
                        break 'outer;
                    }

                    game.current_level_pack_index.replace_with(|i| *i - 1);
                },
                keys::UP => {
                    if *game.current_level_pack_index.borrow() <= 24 {
                        break 'outer;
                    }

                    game.current_level_pack_index.replace_with(|i| *i - 24);
                },
                keys::RIGHT => {
                    if *game.current_level_pack_index.borrow() + 1 >= game.get_level_pack_count() {
                        break 'outer;
                    }

                    game.current_level_pack_index.replace_with(|i| *i + 1);
                },
                keys::DOWN => {
                    if *game.current_level_pack_index.borrow() + 24 >= game.get_level_pack_count() {
                        break 'outer;
                    }

                    game.current_level_pack_index.replace_with(|i| *i + 24);
                },

                keys::ENTER => {
                    //Set selected level
                    if game.get_current_level_pack().as_ref().unwrap().min_level_not_completed() ==
                            game.get_current_level_pack().as_ref().unwrap().level_count() {
                        game.set_level_index(0);
                    }else {
                        game.set_level_index(game.get_current_level_pack().as_ref().unwrap().min_level_not_completed());
                    }

                    game.set_screen(ScreenId::SelectLevel);
                },

                _ => {},
            }
        }
    }

    fn on_mouse_pressed(&self, game: &Game, column: usize, row: usize) {
        if row == 0 {
            return;
        }

        let level_pack_index = column/3 + (row - 1)/2*24;
        if level_pack_index < game.get_level_pack_count() {
            game.set_level_pack_index(level_pack_index);
            self.on_key_pressed(game, keys::ENTER);
        }
    }
}

pub struct ScreenSelectLevel {
    selected_level: RefCell<usize>,
}

impl ScreenSelectLevel {
    pub fn new() -> Self {
        Self {
            selected_level: Default::default(),
        }
    }
}

impl Screen for ScreenSelectLevel {
    fn draw(&self, game: &Game, console: &Console) {
        console.reset_color();
        console.set_underline(true);
        console.draw_text(format!("Select a level (Level pack \"{}\"):", game.get_current_level_pack().unwrap().id()));
        console.set_underline(false);

        let level_count = game.get_current_level_pack().as_ref().unwrap().level_count();

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

            let min_level_not_completed = game.get_current_level_pack().as_ref().unwrap().min_level_not_completed();
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
        let x = (*self.selected_level.borrow()%24)*3;
        let y = 1 + (*self.selected_level.borrow()/24)*2;

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
        let selected_level = *self.selected_level.borrow();
        if selected_level + 1 < 100 {
            console.draw_text(format!("{:02}", selected_level + 1));
        }else {
            console.draw_text(format!("{}", (b'A' + (selected_level as u8 + 1 - 100) / 10) as char));
            console.draw_text(format!("{}", (selected_level + 1) % 10));
        }
        console.set_cursor_pos(1, y + 2);
        console.draw_text("Best time     : ");
        match game.get_current_level_pack().as_ref().unwrap().levels().get(selected_level).unwrap().best_time() {
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
        match game.get_current_level_pack().as_ref().unwrap().levels().get(selected_level).unwrap().best_moves() {
            None => console.draw_text("XXXX"),
            Some(best_moves) => {
                console.draw_text(format!("{:04}", best_moves));
            },
        }
    }

    fn on_key_pressed(&self, game: &Game, key: i32) {
        if key == keys::ESC {
            game.set_screen(ScreenId::SelectLevelPack);

            return;
        }

        if key == keys::F1 {
            game.open_help_page();

            return;
        }

        'outer: {
            match key {
                keys::LEFT => {
                    if *self.selected_level.borrow() == 0 {
                        break 'outer;
                    }

                    self.selected_level.replace_with(|i| *i - 1);
                },
                keys::UP => {
                    if *self.selected_level.borrow() < 24 {
                        break 'outer;
                    }

                    self.selected_level.replace_with(|i| *i - 24);
                },
                keys::RIGHT => {
                    if *self.selected_level.borrow() + 1 >= game.get_current_level_pack().
                            as_ref().unwrap().level_count() {
                        break 'outer;
                    }

                    self.selected_level.replace_with(|i| *i + 1);
                },
                keys::DOWN => {
                    if *self.selected_level.borrow() + 24 >= game.get_current_level_pack().
                            as_ref().unwrap().level_count() {
                        break 'outer;
                    }

                    self.selected_level.replace_with(|i| *i + 24);
                },

                keys::ENTER if *self.selected_level.borrow() <= game.get_current_level_pack().
                        as_ref().unwrap().min_level_not_completed() => {
                    game.set_level_index(*self.selected_level.borrow());
                    game.set_screen(ScreenId::InGame);
                },

                _ => {},
            }
        }
    }

    fn on_mouse_pressed(&self, game: &Game, column: usize, row: usize) {
        if row == 0 {
            return;
        }

        let level_index = column/3 + (row - 1)/2*24;
        if level_index < game.get_current_level_pack().as_ref().unwrap().level_count() {
            self.selected_level.replace(level_index);
            self.on_key_pressed(game, keys::ENTER);
        }
    }

    fn on_set_screen(&self, game: &Game) {
        self.selected_level.replace(game.get_level_index());
    }
}

pub struct ScreenInGame {
    time_start_in_menu: RefCell<Option<SystemTime>>,
    time_start: RefCell<Option<SystemTime>>,
    time_millis: RefCell<u32>,
    time_sec: RefCell<u32>,
    time_min: RefCell<u32>,

    moves: RefCell<u32>,
    old_moves: RefCell<u32>,

    player_pos: RefCell<(usize, usize)>,
    old_player_pos: RefCell<(usize, usize)>,

    level_now: RefCell<Option<Level>>,
    level_now_last_step: RefCell<Option<Level>>,

    continue_flag: RefCell<bool>,
    continue_level_add_flag: RefCell<bool>,
    game_over_flag: RefCell<bool>,
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
            continue_level_add_flag: Default::default(),
            game_over_flag: Default::default(),
        }
    }

    pub fn start_level(&self, level: &Level) {
        //Reset stats
        self.time_start.replace(None);
        self.time_millis.replace(0);
        self.time_sec.replace(0);
        self.time_min.replace(0);

        self.moves.replace(0);

        self.level_now.replace(Some(level.clone()));
        self.level_now_last_step.replace(Some(level.clone()));

        'outer:
        for i in 0..level.width() {
            for j in 0..level.height() {
                if let Some(tile) = level.get_tile(i, j) {
                    if *tile == Tile::Player {
                        self.player_pos.replace((i, j));
                        self.old_player_pos.replace((i, j));

                        break 'outer;
                    }
                }
            }
        }
    }

    fn draw_tutorial_level_text(&self, game: &Game, console: &Console) {
        //Draw special help text for tutorial levels (tutorial pack and tutorial levels in special pack)
        if game.get_level_pack_index() == 0 { //Tutorial pack
            console.reset_color();
            match *game.current_level_index.borrow() {
                0 => {
                    if *self.continue_flag.borrow() {
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
                    if *self.game_over_flag.borrow() {
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
        }else if game.get_level_pack_index() == 2 { //Built-in special pack
            console.reset_color();
            match *game.current_level_index.borrow() {
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
    fn draw(&self, game: &Game, console: &Console) {
        console.reset_color();
        console.draw_text(format!("Pack: {:02}", game.get_level_pack_index() + 1));

        console.set_cursor_pos(((Game::CONSOLE_MIN_WIDTH - 9) as f64 * 0.25) as usize, 0);
        console.draw_text("Level: ");
        let level = *game.current_level_index.borrow() + 1;
        if level < 100 {
            console.draw_text(format!("{:02}", level));
        }else {
            console.draw_text(format!("{}", (b'A' + (level as u8 + 1 - 100) / 10) as char));
            console.draw_text(format!("{}", (level + 1) % 10));
        }

        console.set_cursor_pos(((Game::CONSOLE_MIN_WIDTH - 11) as f64 * 0.75) as usize, 0);
        console.draw_text(format!("Moves: {:04}", *self.moves.borrow()));

        console.set_cursor_pos(Game::CONSOLE_MIN_WIDTH - 15, 0);
        console.draw_text(format!(
            "Time: {:02}:{:02}.{:03}",
            self.time_min.borrow(),
            self.time_sec.borrow(),
            self.time_millis.borrow(),
        ));

        if *self.continue_flag.borrow() {
            console.set_cursor_pos(((Game::CONSOLE_MIN_WIDTH - 16) as f64 * 0.5) as usize, 0);
            console.draw_text("Level completed!");
        }

        if *self.game_over_flag.borrow() {
            console.set_cursor_pos(((Game::CONSOLE_MIN_WIDTH - 13) as f64 * 0.5) as usize, 0);
            console.draw_text("You have won!");
        }

        if let Some(ref level) = *self.level_now.borrow() {
            let x_offset = ((Game::CONSOLE_MIN_WIDTH - level.width()) as f64 * 0.5) as usize;
            let y_offset = 1;

            level.draw(console, x_offset, y_offset, game.is_player_background());

            self.draw_tutorial_level_text(game, console);
        }
    }

    fn update(&self, game: &Game) {
        if game.is_dialog_opened() || *self.game_over_flag.borrow() || *self.continue_flag.borrow() {
            return;
        }

        if let Some(time_start) = self.time_start.borrow().as_ref() {
            let time_current = SystemTime::now();

            let diff = time_current.duration_since(*time_start).
                    expect("Time manipulation detected (Start time is in the future)!").
                    as_millis();

            self.time_millis.replace((diff % 1000) as u32);
            self.time_sec.replace((diff / 1000 % 60) as u32);
            self.time_min.replace((diff / 1000 / 60) as u32);
        }
    }

    fn on_key_pressed(&self, game: &Game, key: i32) {
        if game.is_dialog_opened() {
            if key == b'y' as i32 {
                game.close_dialog();

                self.continue_flag.replace(false);
                self.continue_level_add_flag.replace(false);
                self.game_over_flag.replace(false);

                game.set_screen(ScreenId::SelectLevel);
            }else if key == b'n' as i32 {
                game.close_dialog();

                self.on_continue(game);
            }

            return;
        }

        if key == keys::ESC {
            if *self.game_over_flag.borrow() {
                self.continue_flag.replace(false);
                self.continue_level_add_flag.replace(false);
                self.game_over_flag.replace(false);

                game.set_screen(ScreenId::SelectLevel);

                return;
            }

            self.time_start_in_menu.replace(Some(SystemTime::now()));

            game.open_dialog(Dialog::new("Back to level selection?"));

            return;
        }

        if key == keys::F1 {
            self.time_start_in_menu.replace(Some(SystemTime::now()));

            game.open_help_page();

            return;
        }

        let Some(mut level_pack) = game.get_current_level_pack_mut() else {
            return;
        };

        //Level end
        if *self.continue_flag.borrow() {
            if *self.continue_level_add_flag.borrow() {
                if *game.current_level_index.borrow() >= level_pack.min_level_not_completed() {
                    level_pack.set_min_level_not_completed(*game.current_level_index.borrow() + 1);
                }

                self.continue_level_add_flag.replace(false);
            }

            if key == keys::ENTER {
                self.continue_flag.replace(false);

                //All levels completed
                if *game.current_level_index.borrow() + 1 == level_pack.level_count() {
                    self.game_over_flag.replace(true);

                    return;
                }else {
                    game.current_level_index.replace_with(|current_level_index| *current_level_index + 1);
                }

                self.start_level(level_pack.levels()[*game.current_level_index.borrow()].level());
            }else if key == 'r' as i32 {
                self.continue_flag.replace(false);

                self.start_level(level_pack.levels()[*game.current_level_index.borrow()].level());
            }

            return;
        }

        //One step back
        if key == 'z' as i32 {
            self.level_now.swap(&self.level_now_last_step);

            //Reset move count
            self.moves.swap(&self.old_moves);

            //Reset player pos
            self.player_pos.swap(&self.old_player_pos);
        }

        //Reset
        if key == 'r' as i32 {
            self.start_level(level_pack.levels()[*game.current_level_index.borrow()].level());
        }

        if console_lib::is_arrow_key(key) {
            let level_now_before_move = self.level_now.borrow().clone();

            let player_pos_tmp = *self.player_pos.borrow();

            //Set players old position to old level data
            let mut tile = level_pack.levels()[*game.current_level_index.borrow()].level().
                    get_tile(player_pos_tmp.0, player_pos_tmp.1).unwrap().clone();
            if tile == Tile::Player || tile == Tile::Box || tile == Tile::Key || tile == Tile::LockedDoor {
                tile = Tile::Empty;
            }else if tile == Tile::BoxInGoal || tile == Tile::KeyInGoal {
                tile = Tile::Goal;
            }

            self.level_now.borrow_mut().as_mut().unwrap().set_tile(player_pos_tmp.0, player_pos_tmp.1, tile);

            self.time_start.borrow_mut().get_or_insert_with(SystemTime::now);

            let mut has_won = false;
            match key {
                keys::LEFT => {
                    let tile = self.level_now.borrow().as_ref().unwrap().get_tile(player_pos_tmp.0 - 1, player_pos_tmp.1).unwrap().clone();
                    match tile {
                        Tile::Empty | Tile::Goal | Tile::OneWayLeft => {
                            self.player_pos.replace_with(|(x, y)| (*x - 1, *y));
                        },
                        Tile::Box | Tile::BoxInGoal | Tile::Key | Tile::KeyInGoal if
                        self.level_now.borrow_mut().as_mut().unwrap().move_box_or_key(
                            level_pack.levels().get(*game.current_level_index.borrow()).unwrap().level(),
                            &mut has_won, player_pos_tmp.0 - 1, player_pos_tmp.1, -1, 0
                        ) => {
                            self.player_pos.replace_with(|(x, y)| (*x - 1, *y));
                        },
                        _ => {},
                    }
                },
                keys::UP => {
                    let tile = self.level_now.borrow().as_ref().unwrap().get_tile(player_pos_tmp.0, player_pos_tmp.1 - 1).unwrap().clone();
                    match tile {
                        Tile::Empty | Tile::Goal | Tile::OneWayUp => {
                            self.player_pos.replace_with(|(x, y)| (*x, *y - 1));
                        },
                        Tile::Box | Tile::BoxInGoal | Tile::Key | Tile::KeyInGoal if
                        self.level_now.borrow_mut().as_mut().unwrap().move_box_or_key(
                            level_pack.levels().get(*game.current_level_index.borrow()).unwrap().level(),
                            &mut has_won, player_pos_tmp.0, player_pos_tmp.1 - 1, 0, -1
                        ) => {
                            self.player_pos.replace_with(|(x, y)| (*x, *y - 1));
                        },
                        _ => {},
                    }
                },
                keys::RIGHT => {
                    let tile = self.level_now.borrow().as_ref().unwrap().get_tile(player_pos_tmp.0 + 1, player_pos_tmp.1).unwrap().clone();
                    match tile {
                        Tile::Empty | Tile::Goal | Tile::OneWayRight => {
                            self.player_pos.replace_with(|(x, y)| (*x + 1, *y));
                        },
                        Tile::Box | Tile::BoxInGoal | Tile::Key | Tile::KeyInGoal if
                        self.level_now.borrow_mut().as_mut().unwrap().move_box_or_key(
                            level_pack.levels().get(*game.current_level_index.borrow()).unwrap().level(),
                            &mut has_won, player_pos_tmp.0 + 1, player_pos_tmp.1, 1, 0
                        ) => {
                            self.player_pos.replace_with(|(x, y)| (*x + 1, *y));
                        },
                        _ => {},
                    }
                },
                keys::DOWN => {
                    let tile = self.level_now.borrow().as_ref().unwrap().get_tile(player_pos_tmp.0, player_pos_tmp.1 + 1).unwrap().clone();
                    match tile {
                        Tile::Empty | Tile::Goal | Tile::OneWayDown => {
                            self.player_pos.replace_with(|(x, y)| (*x, *y + 1));
                        },
                        Tile::Box | Tile::BoxInGoal | Tile::Key | Tile::KeyInGoal if
                        self.level_now.borrow_mut().as_mut().unwrap().move_box_or_key(
                            level_pack.levels().get(*game.current_level_index.borrow()).unwrap().level(),
                            &mut has_won, player_pos_tmp.0, player_pos_tmp.1 + 1, 0, 1
                        ) => {
                            self.player_pos.replace_with(|(x, y)| (*x, *y + 1));
                        },
                        _ => {},
                    }
                },
                _ => {},
            }

            //Set player to new position
            self.level_now.borrow_mut().as_mut().unwrap().set_tile(self.player_pos.borrow().0, self.player_pos.borrow().1, Tile::Player);

            //Copy level to last step if change
            if *self.player_pos.borrow() != player_pos_tmp {
                self.old_moves.replace(*self.moves.borrow());
                self.moves.replace_with(|moves| *moves + 1);

                self.old_player_pos.replace(player_pos_tmp);
                self.level_now_last_step.replace(level_now_before_move);
            }

            if has_won {
                self.continue_flag.replace(true);
                self.continue_level_add_flag.replace(true);

                //Update best scores
                let time = *self.time_millis.borrow() as u64 + 1000 * *self.time_sec.borrow() as u64 + 60000 * *self.time_min.borrow() as u64;
                let moves = *self.moves.borrow();

                level_pack.update_stats(*game.current_level_index.borrow(), time, moves);

                //TODO replace with error popup
                level_pack.save_save_game().expect("Can not save save game");
            }
        }
    }

    fn on_continue(&self, _: &Game) {
        if *self.game_over_flag.borrow() || *self.continue_flag.borrow() ||
                self.time_start.borrow().is_none() || self.time_start_in_menu.borrow().is_none() {
            return;
        }

        let diff = SystemTime::now().duration_since(*self.time_start_in_menu.replace(None).as_ref().unwrap()).
                expect("Time manipulation detected (Start time is in the future)!");

        self.time_start.replace_with(|time_start| {
            time_start.map(|time_start| time_start + diff)
        });
    }

    fn on_set_screen(&self, game: &Game) {
        self.start_level(game.get_current_level_pack().as_ref().unwrap().levels().get(
            game.get_level_index()).unwrap().level());
    }
}

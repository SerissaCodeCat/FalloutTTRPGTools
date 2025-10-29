use color_eyre::owo_colors::OwoColorize;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;

pub mod energy_weapons_list;
pub mod heavy_weapons_list;
pub mod melee_weapons_list;
pub mod small_guns_list;
pub mod utility;
pub mod weapon;
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init(); //allowing the app complete
    let app_result = App::default().run(&mut terminal);
    ratatui::restore(); //return control of the Terminal to standard.
    return app_result;
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    counter: u8,
}
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        return Ok(());
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            //for the purposes here, we will only act on key PRESS, not release, or other.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            //if it is a differnt type of event ( _ is an all match) do nothing.
            _ => {}
        };
        return Ok(());
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Fallout TTRPG Helper");
        let instructions = Line::from(vec![
            "Decrement".into(),
            "<left>".blue().bold(),
            "Increment".into(),
            "<right>".green().bold(),
            "quit".into(),
            "<Esc>".red().bold(),
        ]);
        let block = Block::bordered()
            .title(title.left_aligned())
            .title_bottom(instructions.left_aligned())
            .border_set(border::THICK);
        let counter_text = Text::from(vec![Line::from(vec![
            "value".into(),
            self.counter.to_string().yellow(),
        ])]);
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

//impl App {
//    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
//        while !self.exit {
//            terminal.draw(render_callback: | frame: &mut Frame<'_>| self.draw(frame))?;
//        }
//        return Ok(());
//    }
//
//    fn draw(&self, frame: &mut Frame){
//        frame.render_widget(widget: self, frame.area());
//    }
//}
//impl Widget for &App {
//    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
//        where
//            Self: Sized {
//        //render title in the top of layout
//        Line::from("FALLOUT TTRPG HELPER").bold().render(area, buf);
//    }
//}
//fn weapon_loot_generator() {
//    let mut gun_list = small_guns_list::create_small_gun_list();
//    gun_list.append(&mut energy_weapons_list::create_energy_weapon_list());
//    gun_list.append(&mut heavy_weapons_list::create_heavy_weapon_list());
//    gun_list.append(&mut melee_weapons_list::create_melee_weapon_list());
//    let random_weapon = random_range(0..gun_list.len());
//    print!("{}", gun_list[random_weapon]);
//}
//fn shop_weapon_inventory_generator() {
//    println!("under development");
//}
//fn player_weapon_planner() {
//    println!("under development")
//}
//fn location_and_map_generator() {
//    println!("under development.");
//}
//fn exit_function() {
//    println!("data saving goes here, and then exit");
//    std::process::exit(0);
//}

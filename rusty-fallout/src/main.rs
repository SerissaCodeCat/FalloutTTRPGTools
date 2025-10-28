use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::event::{self, Event};
use rand::random_range;
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub mod energy_weapons_list;
pub mod heavy_weapons_list;
pub mod melee_weapons_list;
pub mod small_guns_list;
pub mod utility;
pub mod weapon;
fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init(); //allowing the app complete
    let result = run(terminal);
    ratatui::restore(); //return control of the Terminal to standard.
    return result;
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break return Ok(());
        }
    }
}
fn render(frame: &mut Frame) {
    frame.render_widget("FALLOUT TTRPG HELPER", frame.area());
}

//pub struct App {
//    exit: bool,
//}
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
//fn render(frame: &mut Frame) {
//    let outer_layout = Layout::default()
//        .direction(Direction::Vertical)
//        .margin(1)
//        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
//        .split(frame.area());
//
//    frame.render_widget(
//        Paragraph::new("Main Options").block(
//            Block::new().title("MAIN OPTIONS").bold().fg(Color::Blue).borders(Borders::ALL),
//        ),
//        outer_layout[0],
//    );
//    frame.render_widget(
//        Paragraph::new("Working Area").block(
//            Block::new().title("WORKING AREA").bold(), //.borders(Borders::ALL),
//        ),
//        outer_layout[1],
//    );
//}

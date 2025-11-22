use color_eyre::owo_colors::OwoColorize;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, 
    layout::{Layout, Rect}, 
    prelude::*, style::Stylize, symbols::border, 
    text::{Line, Text}, 
    widgets::{Block, Borders, Paragraph, Widget, List, ListItem, ListState, Tabs}
};
use std::{io, vec};

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


struct MainMenuItems {
    item: Vec<MainMenuList>

}
struct MainMenuList {
    item: String,
}

#[derive(Debug)]
pub struct App {
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        App { exit: false }
    }
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
        let outer_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .margin(2)
        .constraints([
            Constraint::Percentage(20), 
            Constraint::Percentage(80)
            ]
            .as_ref())
        .split(frame.area());

        let inner_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .margin(0)
        .constraints([
                ratatui::layout::Constraint::Percentage(20),
                ratatui::layout::Constraint::Percentage(80),
            ]
            .as_ref(),
        )
        .split(outer_layout[1]);
        let mut main_menu_state = ListState::default();
        main_menu_state.select(Some(0));
        let main_menu_items = vec![
            "LOOT GENERATION",
            "NPC GENERATION",
            "MERCHANT GENERATION",
            "LOCATION GENERATION",
            "ENCOUNTER GENERATION",
        ];
        let main_menu_list = List::new(main_menu_items)
            .block(Block::new().borders(Borders::ALL).green().title("MAIN MENU"))
            .style(Style::new().green())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        frame.render_stateful_widget(main_menu_list, outer_layout[0], &mut main_menu_state);
        frame.render_widget(Paragraph::new("Sub Menu")
            .block(Block::new().borders(Borders::ALL).green().title("SUB MENU")), 
            inner_layout[0]);
        frame.render_widget(Paragraph::new("Working Area")
            .block(Block::new().borders(Borders::ALL).green().title("WORKING AREA")), 
            inner_layout[1]);
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
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
    }

    fn decrement_counter(&mut self) {
    }
}

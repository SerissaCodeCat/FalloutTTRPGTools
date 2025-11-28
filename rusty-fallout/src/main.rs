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
    let mut master_main_menu_state = ListState::default();
    master_main_menu_state.select(Some(0));
    let app_result = App::default().run(&mut terminal, &mut master_main_menu_state);
    ratatui::restore(); //return control of the Terminal to standard.
    return app_result;
}

#[derive(Debug)]
pub struct App <'a>{
    exit: bool,
    main_menu_list: List<'a>,
}


impl Default for App<'_> {
    fn default() -> Self {
        App { 
            exit: false, 
            main_menu_list: List::new(vec![
                "LOOT GENERATION",
                "NPC GENERATION",
                "MERCHANT GENERATION",
                "LOCATION GENERATION",
                "ENCOUNTER GENERATION",
                "EXIT SESSION",]).block(Block::new().borders(Borders::ALL).green().title("MAIN MENU"))
            .style(Style::new().green())
            .highlight_style(Style::new().bg(Color::Green).black().bold())
            .repeat_highlight_symbol(true),
        }
    }
}

impl App<'_>{
    
    pub fn run(&mut self, terminal: &mut DefaultTerminal, main_menu_state: &mut ListState ) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame, main_menu_state))?;
            self.handle_events(main_menu_state)?;
        }
        return Ok(());
    }

    fn draw(&self, frame: &mut Frame, main_menu_state: &mut ListState) {
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

        //let mut main_menu_state = ListState::default();
        //main_menu_state.select(Some(0));
        frame.render_stateful_widget(&self.main_menu_list, outer_layout[0], main_menu_state);
        frame.render_widget(Paragraph::new("Sub Menu")
            .block(Block::new().borders(Borders::ALL).green().title("SUB MENU")), 
            inner_layout[0]);
        frame.render_widget(Paragraph::new("Working Area")
            .block(Block::new().borders(Borders::ALL).green().title("WORKING AREA")), 
            inner_layout[1]);
    }

    fn handle_events(&mut self, main_menu_state: &mut ListState) -> io::Result<()> {
        match event::read()? {
            //for the purposes here, we will only act on key PRESS, not release, or other.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event, main_menu_state);
            }
            //if it is a differnt type of event ( _ is an all match) do nothing.
            _ => {}
        };
        return Ok(());
    }
    fn handle_key_event(&mut self, key_event: KeyEvent, main_menu_state: &mut ListState) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char('j') => main_menu_state.select_next(),
            KeyCode::Char('k') => main_menu_state.select_previous(),
            KeyCode::Enter => {
                //handle selection
                if let Some(selected) = main_menu_state.selected() {
                    match selected {
                        0 => {
                            //LOOT GENERATION selected
                        }
                        1 => {
                            //NPC GENERATION selected
                        }
                        2 => {
                            //MERCHANT GENERATION selected
                        }
                        3 => {
                            //LOCATION GENERATION selected
                        }
                        4 => {
                            //ENCOUNTER GENERATION selected
                        }
                        5 => {
                            //EXIT SESSION selected
                            self.exit();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

}

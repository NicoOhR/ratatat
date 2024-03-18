use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*,*},
};

use chrono::{DateTime, Local};

pub struct Date{
    year: u8,
    month: u8, 
    day: u8,
}

pub enum CurrentScreen {
    Main,
    //UserPage, add later
    MoviePage,
    Exiting,
}

pub enum AddingMovie {
    Title,
    Date,
    Rating,
}

pub struct App{
    pub movie_name_input: String,
    pub date_watched_input: Date,
    pub rating: u8,
    pub entry: std::collections::HashMap<String, (Date, u8)>, //should probably make movie info struct
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<AddingMovie>,
}

impl Date{
    //initialize empty date??
    pub fn new() -> Date{
        Date{
            year: 0,
            month: 0,
            day: 0,
        }
    } 
}

impl App {
    pub fn new() -> App{
        App {
            movie_name_input: String::new(),
            date_watched_input: Date::new(),
            rating: 0,
            entry: std::collections::HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

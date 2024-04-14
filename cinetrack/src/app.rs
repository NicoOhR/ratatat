use std::process::Output;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*,*},
};
use serde::{Serialize, Deserialize};




use chrono::{DateTime, TimeZone, Utc};
use std::fmt;

pub enum CurrentScreen {
    Main,
    //UserPage, add later
    MoviePage,
    Exiting,
}


pub struct MovieInfo{
    pub date_watched: DateTime<Utc>,
    pub rating: String,
}

pub enum AddingMovie {
    Title,
    DateWatched,
    Rating,
}

pub struct App{
    pub movie_name_input: String,
    pub date_watched_input: Vec<u8>,
    pub rating_input: String,
    pub entries: std::collections::HashMap<String, MovieInfo>, //should probably make movie info struct
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<AddingMovie>,
}

impl fmt::Display for MovieInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Date: {} Rating: {}", self.date_watched, self.rating)
    }
}

impl Serialize for MovieInfo {
    
}


impl App {
    pub fn new() -> App{
        App {
            movie_name_input: String::new(),
            date_watched_input: Vec::new(),
            rating_input: String::new(),
            entries: std::collections::HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_new_movie(&mut self){
        let new_movie_info = MovieInfo {
            date_watched: self. date_watched_input.clone(),
            rating: self.rating_input.clone(),
        };
        
        self.entries.insert(self.movie_name_input.clone(), new_movie_info);

        self.movie_name_input = String::new();
        self.rating_input = String::new();
        self.date_watched_input = Date::from_ordinal_date(1990, 355).unwrap();

        self.currently_editing = None; 
    }

    pub fn toggle_editing(&mut self){
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode{
                AddingMovie::Title => {
                    self.currently_editing = Some(AddingMovie::Title)
                }
                AddingMovie::Rating => {
                    self.currently_editing = Some(AddingMovie::Rating)
                }
                AddingMovie::DateWatched => {
                    self.currently_editing = Some(AddingMovie::DateWatched)
                }
            };
        } else {
            self.currently_editing = Some(AddingMovie::Title);
        }
    }

    pub fn print_json(&self) -> std::io::Result<()> {
        let movie_list = serde_json::to_string(&self.entries)?;
        println!("{}", movie_list);
        Ok(())
    }

}
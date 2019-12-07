use chrono::*;
use reqwest::*;
use serde_derive::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Deserialize, Debug, Clone)]
pub struct leagueRecord {
    pub wins: u32,
    pub losses: u32,
    pub pct: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct teamInfo {
    pub id: u32,
    pub name: String,
    pub link: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GameTeam {
    pub leagueRecord: leagueRecord,
    pub score: u32,
    pub team: teamInfo,
    pub isWinner: bool,
    pub splitSquad: bool,
    pub seriesNumber: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GameTeams {
    pub away: GameTeam,
    pub home: GameTeam,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Game {
    pub gamePk: u32,
    pub link: String,
    pub gameType: String,
    pub season: String,
    pub gameDate: String,
    pub status: Value,
    pub teams: GameTeams,
    pub decisions: Value,
    pub venue: Value,
    pub content: Value,
    pub isTie: bool,
    pub gameNumber: u32,
    pub publicFacing: bool,
    pub doubleHeader: String,
    pub gamedayType: String,
    pub tiebreaker: String,
    pub calendarEventID: String,
    pub seasonDisplay: String,
    pub dayNight: String,
    pub scheduledInnings: u32,
    pub inningBreakLength: u32,
    pub gamesInSeries: u32,
    pub seriesGameNumber: u32,
    pub seriesDescription: String,
    pub recordSource: String,
    pub ifNecessary: String,
    pub ifNecessaryDescription: String,
}

pub struct MlbApi {}

impl MlbApi {
    pub fn get_items() -> Vec<Game> {
        // let resp: HashMap<String, serde_json::Value> = reqwest::blocking::get("http://statsapi.mlb.com/api/v1/schedule?hydrate=game(content(editorial(recap))),decisions&date=2018-06-10&sportId=1")
        // .unwrap()
        // .json()
        // .unwrap();
        // println!("{:#?}", resp.keys());

        let json = read_to_string("./schedule.json").unwrap();
        // let json: String = reqwest::blocking::get("http://statsapi.mlb.com/api/v1/schedule?hydrate=game(content(editorial(recap))),decisions&date=2018-06-10&sportId=1").unwrap().text().unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        // let game :Game = serde_json::from_value(parsed["dates"][0]["games"][0].to_owned()).unwrap();
        // println!("{:#?}", game);
        serde_json::from_value(parsed["dates"][0]["games"].to_owned()).unwrap()
    }
}

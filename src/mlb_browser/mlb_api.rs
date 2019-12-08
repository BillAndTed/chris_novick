use chrono::*;
use reqwest::*;
use serde_derive::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{read_to_string};
use std::fs::{File};
use std::io::{copy, Read, Seek, SeekFrom};

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct ContentRecapPhotoCutItem {
    pub aspectRatio: String,
    pub width: u32,
    pub height: u32,
    pub src: String,
    pub at2x: String,
    pub at3x: String,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct ContentRecapPhotoCuts {
    #[serde(rename = "1920x1080")]
    pub _1920x1080: ContentRecapPhotoCutItem,
    #[serde(rename = "1440x810")]
    pub _1440x810: ContentRecapPhotoCutItem,
    #[serde(rename = "1280x720")]
    pub _1280x720: ContentRecapPhotoCutItem,
    #[serde(rename = "960x540")]
    pub _960x540: ContentRecapPhotoCutItem,
    #[serde(rename = "800x448")]
    pub _800x448: ContentRecapPhotoCutItem,
    #[serde(rename = "720x405")]
    pub _720x405: ContentRecapPhotoCutItem,
    #[serde(rename = "684x385")]
    pub _684x385: ContentRecapPhotoCutItem,
    #[serde(rename = "640x360")]
    pub _640x360: ContentRecapPhotoCutItem,
    #[serde(rename = "496x279")]
    pub _496x279: ContentRecapPhotoCutItem,
    #[serde(rename = "480x270")]
    pub _480x270: ContentRecapPhotoCutItem,
    #[serde(rename = "430x242")]
    pub _430x242: ContentRecapPhotoCutItem,
    #[serde(rename = "400x224")]
    pub _400x224: ContentRecapPhotoCutItem,
    #[serde(rename = "320x180")]
    pub _320x180: ContentRecapPhotoCutItem,
    #[serde(rename = "270x154")]
    pub _270x154: ContentRecapPhotoCutItem,
    #[serde(rename = "248x138")]
    pub _248x138: ContentRecapPhotoCutItem,
    #[serde(rename = "215x121")]
    pub _215x121: ContentRecapPhotoCutItem,
    #[serde(rename = "209x118")]
    pub _209x118: ContentRecapPhotoCutItem,
    #[serde(rename = "135x77")]
    pub _135x77: ContentRecapPhotoCutItem,
    #[serde(rename = "124x70")]
    pub _124x70: ContentRecapPhotoCutItem,
    #[serde(rename = "222x168")]
    pub _222x168: ContentRecapPhotoCutItem,
    #[serde(rename = "192x144")]
    pub _192x144: ContentRecapPhotoCutItem,
    #[serde(rename = "148x112")]
    pub _148x112: ContentRecapPhotoCutItem,
    #[serde(rename = "96x72")]
    pub _96x72: ContentRecapPhotoCutItem,
    #[serde(rename = "74x56")]
    pub _74x56: ContentRecapPhotoCutItem,
    #[serde(rename = "1920x810")]
    pub _1920x810: ContentRecapPhotoCutItem,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct ContentRecapPhotos {
    pub title: String,
    pub altText: String,
    pub cuts: ContentRecapPhotoCuts,
}
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct ContentRecapItems {
    pub r#type: String,
    pub state: String,
    pub date: String,
    pub id: String,
    pub headline: String,
    pub subhead: String,
    pub seoTitle: String,
    pub seoKeywords: String,
    pub seoDescription: String,
    pub slug: String,
    pub commenting: bool,
    pub photo: ContentRecapPhotos,
    pub image: Value,
    pub tokenData: Value,
    pub blurb: String,
    pub body: String,
    pub contributor: Value,
    pub keywordsDisplay: Value,
    pub keywordsAll: Value,
    pub approval: String,
    pub canonical: String,
    pub dataURI: String,
    pub primaryKeyword: Value,
    pub media: Value,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct ContentRecap {
    pub home: ContentRecapItems,
    pub away: ContentRecapItems,
    pub mlb: ContentRecapItems,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct ContentEditorial {
    pub recap: ContentRecap,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    pub link: String,
    pub editorial: ContentEditorial,
    pub media: Value,
    pub highlights: Value,
    pub summary: Value,
    pub gameNotes: Value,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct leagueRecord {
    pub wins: u32,
    pub losses: u32,
    pub pct: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct teamInfo {
    pub id: u32,
    pub name: String,
    pub link: String,
}

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
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
    pub content: Content,
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

impl Game {
    pub fn get_recap(&self) -> (&String, &String) {
        (
            &self.content.editorial.recap.mlb.photo.title,
            &self.content.editorial.recap.mlb.photo.cuts._640x360.src,
        )
    }

    pub fn get_img(url: String, id: String) -> Vec<u8> {
        // include_bytes!("../assets/cut.jpg")

        let mut response = reqwest::blocking::get(&url).unwrap();

        let mut dest = File::create(&id).unwrap();
        copy(&mut response, &mut dest).unwrap();
        let mut buffer = Vec::with_capacity(response.content_length().unwrap() as usize);
        // copy(&mut response, &mut buffer).unwrap();
        let mut readfile = File::open(&id).unwrap();
        readfile.read_to_end(&mut buffer).unwrap();
        buffer.to_owned()
        
    }
}

pub struct MlbApi {}

impl MlbApi {
    pub fn get_items() -> Vec<Game> {
        // let resp: HashMap<String, serde_json::Value> = reqwest::blocking::get("http://statsapi.mlb.com/api/v1/schedule?hydrate=game(content(editorial(recap))),decisions&date=2018-06-10&sportId=1")
        // .unwrap()
        // .json()
        // .unwrap();
        // println!("{:#?}", resp.keys());

        // let json = read_to_string("src/assets/schedule.json").unwrap();
        let json: String = reqwest::blocking::get("http://statsapi.mlb.com/api/v1/schedule?hydrate=game(content(editorial(recap))),decisions&date=2018-06-10&sportId=1").unwrap().text().unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        // let game :Game = serde_json::from_value(parsed["dates"][0]["games"][0].to_owned()).unwrap();
        // println!("{:#?}", game);
        serde_json::from_value(parsed["dates"][0]["games"].to_owned()).unwrap()
    }
}

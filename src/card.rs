use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::{collections::HashMap, error::Error, fmt};
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_test_json() {
        let json_str: String = read_to_string("test.json").unwrap();
        let _test_result: ResponseList = serde_json::from_str(&json_str).unwrap();
    }

    #[test]
    fn test_tasigur_colour() {
        let json_str: String = read_to_string("test.json").unwrap();
        let response: ResponseList = serde_json::from_str(&json_str).unwrap();
        let tasigur = response.card_or().unwrap();
        let tas_colours = &tasigur.colors;
        let tas_identity = &tasigur.color_identity;
        assert!(tas_colours.first().unwrap() == &'B');
        assert!(tas_identity.iter().zip(vec!['B', 'G', 'U']).all(|(a, b)| a == &b));
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct CardObject {
    #[serde(default)]
    pub arena_id: Option<u32>,
    pub id: Uuid, //UUID
    pub lang: String,
    #[serde(default)]
    pub mtgo_id: Option<u32>,
    #[serde(default)]
    pub mtgo_foil_id: Option<u32>,
    #[serde(default)]
    pub multiverse_ids: Vec<u32>,
    #[serde(default)]
    pub tcgplayer_id: Option<u32>,
    #[serde(default)]
    pub tcgplayer_etched_id: Option<u32>,
    #[serde(default)]
    pub cardmarket_id: Option<u32>,
    pub object: String,
    pub layout: String,
    #[serde(default)]
    pub oracle_id: Option<Uuid>, //UUID
    pub prints_search_uri: URI, //URI
    pub rulings_uri: URI,       //URI
    pub scryfall_uri: URI,      //URI
    pub uri: URI,               //URI
    #[serde(default)]
    pub all_parts: Vec<RelatedCardObject>,
    #[serde(default)]
    pub card_faces: Vec<CardFace>,
    pub cmc: Number,
    pub color_identity: Vec<char>,
    #[serde(default)]
    pub color_indicator: Vec<char>,
    #[serde(default)]
    pub colors: Vec<char>,
    #[serde(default)]
    pub defence: Option<String>,
    #[serde(default)]
    pub edh_rank: Option<u32>,
    #[serde(default)]
    pub hand_modifier: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub legalities: HashMap<String, String>,
    #[serde(default)]
    pub life_modifier: Option<String>,
    #[serde(default)]
    pub loyalty: Option<String>,
    #[serde(default)]
    pub mana_cost: Option<String>,
    pub name: String,
    #[serde(default)]
    pub oracle_text: Option<String>,
    #[serde(default)]
    pub penny_rank: Option<u32>,
    #[serde(default)]
    pub power: Option<String>,
    #[serde(default)]
    pub produced_mana: Vec<char>,
    pub reserved: bool,
    #[serde(default)]
    pub toughness: Option<String>,
    pub type_line: String,
    #[serde(default)]
    pub artist: Option<String>,
    #[serde(default)]
    pub artist_ids: Vec<Uuid>, //UUID
    #[serde(default)]
    pub attraction_lights: Vec<String>, //unsure
    pub booster: bool,
    pub border_color: String,
    #[serde(default)]
    pub card_back_id: Option<Uuid>, //UUID
    pub collector_number: String,
    #[serde(default)]
    pub content_warning: Option<bool>,
    pub digital: bool,
    pub finishes: Vec<String>,
    #[serde(default)]
    pub flavor_name: Option<String>,
    #[serde(default)]
    pub frame_effects: Vec<String>,
    pub frame: String,
    pub full_art: bool,
    #[serde(default)]
    pub games: Vec<String>,
    pub highres_image: bool,
    #[serde(default)]
    pub illustration_id: Option<Uuid>, //UUID
    pub image_status: String,
    #[serde(default)]
    pub image_uris: HashMap<String, URI>,
    pub oversized: bool,
    pub prices: HashMap<String, Option<String>>,
    #[serde(default)]
    pub printed_name: Option<String>,
    #[serde(default)]
    pub printed_text: Option<String>,
    #[serde(default)]
    pub printed_type_line: Option<String>,
    pub promo: bool,
    #[serde(default)]
    pub promo_types: Vec<String>,
    #[serde(default)]
    pub purchase_uris: HashMap<String, String>,
    pub rarity: String,
    pub related_uris: HashMap<String, URI>,
    pub released_at: String, //DateTime
    pub reprint: bool,
    pub scryfall_set_uri: URI,
    pub set_name: String,
    pub set_search_uri: URI,
    pub set_type: String,
    pub set_uri: URI,
    pub set: String,
    pub set_id: Uuid, //UUID
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    #[serde(default)]
    pub variation_of: Option<Uuid>,
    #[serde(default)]
    pub security_stamp: Option<String>,
    #[serde(default)]
    pub watermark: Option<String>,
    #[serde(rename = "preview.previewed_at")]
    #[serde(default)]
    pub previewed_at: Option<String>, //DateTime
    #[serde(rename = "preview.source_uri")]
    #[serde(default)]
    pub source_uri: Option<URI>,
    #[serde(rename = "preview.source")]
    #[serde(default)]
    pub source: Option<String>,
}

impl CardObject {
    pub fn is_nonland(&self) -> bool {
        let has_colour: bool = self.colors.len() > 0;
        let back_side_has_colour: bool = match self.card_faces.last() {
            Some(face) => face.colors.len() > 0,
            None => false,
        };
        let front_side_has_colour: bool = match self.card_faces.first() {
            Some(face) => face.colors.len() > 0,
            None => false,
        };
        let has_mana_cost: bool = match &self.mana_cost {
            Some(s) => s.len() > 0,
            None => false,
        };

        has_colour || has_mana_cost || front_side_has_colour || back_side_has_colour
    }
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct URI(String);

pub struct Colours(u8);

impl<'de> Deserialize<'de> for Colours {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct ColoursVisitor;

        impl<'de> Visitor<'de> for ColoursVisitor {
            type Value = Colours;
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct RelatedCardObject {
    id: Uuid,
    object: String,
    component: String,
    name: String,
    type_line: String,
    uri: URI,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct CardFace {
    #[serde(default)]
    pub artist: Option<String>,
    #[serde(default)]
    pub artist_id: Option<Uuid>,
    #[serde(default)]
    pub cmc: Option<f64>,
    #[serde(default)]
    pub color_indicator: Vec<char>,
    #[serde(default)]
    pub colors: Vec<char>,
    #[serde(default)]
    pub defence: Option<String>,
    #[serde(default)]
    pub flavor_text: Option<String>,
    #[serde(default)]
    pub illustration_id: Option<Uuid>,
    #[serde(default)]
    pub image_uris: HashMap<String, URI>,
    #[serde(default)]
    pub layout: String,
    #[serde(default)]
    pub loyalty: String,
    pub mana_cost: String,
    pub name: String,
    pub object: String,
    #[serde(default)]
    pub oracle_id: Option<Uuid>,
    #[serde(default)]
    pub oracle_text: Option<String>,
    #[serde(default)]
    pub power: Option<String>,
    #[serde(default)]
    pub printed_name: Option<String>,
    #[serde(default)]
    pub printed_text: Option<String>,
    #[serde(default)]
    pub printed_type_line: Option<String>,
    #[serde(default)]
    pub toughness: Option<String>,
    #[serde(default)]
    pub type_line: Option<String>,
    #[serde(default)]
    pub watermark: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct ResponseList {
    pub object: String,
    pub total_cards: u32,
    pub has_more: bool,
    pub data: Vec<CardObject>,
}

impl ResponseList {
    pub fn card_or<'a>(&'a self) -> Result<&'a CardObject, SearchError> {
        let c = self
            .data
            .first()
            .ok_or::<SearchError>("no first result".into())?;

        if self.total_cards != 1 {
            return Err(SearchError::MultipleHits(
                self.total_cards,
                c.name.to_owned(),
            ));
        };
        return Ok(c);
    }
}

#[derive(Debug, Clone)]
pub enum SearchError {
    MultipleHits(u32, String),
    Other(String),
}

impl From<&str> for SearchError {
    fn from(value: &str) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<String> for SearchError {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchError::MultipleHits(i, s) => write!(f, "found {} cards, did you meen {}?", i, s),
            SearchError::Other(s) => write!(f, "{}", s),
        }
    }
}

impl Error for SearchError {}

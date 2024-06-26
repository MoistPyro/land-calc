use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Deserializer, Serialize};
use serde_json::Number;
use std::{collections::HashMap, fmt};
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    #[should_panic]
    fn test_ser_de() {
        let json_str: String = read_to_string("test.json").unwrap();
        let parsed_json: ResponseList = serde_json::from_str(&json_str).unwrap();
        let test_result: String = serde_json::to_string(&parsed_json).unwrap();

        assert_eq!(json_str, test_result);
    }

    #[test]
    fn test_tasigur_colour() {
        let json_str: String = read_to_string("test.json").unwrap();
        let response: ResponseList = serde_json::from_str(&json_str).unwrap();
        let search_result = response.card_or("".to_string());
        let tasigur = search_result.get_card_ref().unwrap();

        assert!(tasigur.get_colours().0 == 0b00000100);
        assert!(tasigur.get_identity().0 == 0b00001101);
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
    pub prints_search_uri: String, //URI
    pub rulings_uri: String,       //URI
    pub scryfall_uri: String,      //URI
    pub uri: String,               //URI
    #[serde(default)]
    pub all_parts: Vec<RelatedCardObject>,
    #[serde(default)]
    pub card_faces: Vec<CardFace>,
    pub cmc: Number,
    pub color_identity: Colours,
    #[serde(default)]
    pub color_indicator: Vec<char>,
    #[serde(default)]
    pub colors: Colours,
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
    pub image_uris: HashMap<String, String>,
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
    pub related_uris: HashMap<String, String>, // String, URI
    pub released_at: String,                   //DateTime
    pub reprint: bool,
    pub scryfall_set_uri: String, //URI
    pub set_name: String,
    pub set_search_uri: String, //URI
    pub set_type: String,
    pub set_uri: String, //URI
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
    pub source_uri: Option<String>,
    #[serde(rename = "preview.source")]
    #[serde(default)]
    pub source: Option<String>,
}

impl CardObject {
    pub fn is_nonland(&self) -> bool {
        let has_colour: bool = self.colors.0 > 0;
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

    #[allow(dead_code)]
    pub fn get_colours<'a>(&'a self) -> &'a Colours {
        return &self.colors;
    }

    #[allow(dead_code)]
    pub fn get_identity<'a>(&'a self) -> &'a Colours {
        return &self.color_identity;
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
///000WUBRG
pub struct Colours(u8);

impl<'de> Deserialize<'de> for Colours {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ColoursVisitor::new())
    }
}

impl From<u8> for Colours {
    fn from(value: u8) -> Self {
        Colours(value)
    }
}

impl Serialize for Colours {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut length = 0;
        let mut temp_vec: Vec<char> = vec![];
        let names = ['G', 'R', 'B', 'U', 'W'];
        for (i, c) in names.iter().enumerate() {
            if self.0.rotate_right(i as u32) & 0b00000001 == 0b00000001 {
                length += 1;
                temp_vec.push(*c);
            }
        }
        let mut seq = serializer.serialize_seq(Some(length))?;
        assert!(temp_vec.len() > 0);

        for e in temp_vec {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

struct ColoursVisitor;

impl<'de> Visitor<'de> for ColoursVisitor {
    type Value = Colours;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a list of mtg colours")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        {
            let mut r: Vec<char> = vec![];
            while let Some(value) = seq.next_element()? {
                r.push(value)
            }
            let mut c: u8 = 0b00000000;

            for letter in r {
                c = match letter {
                    'W' => c | 0b00010000,
                    'U' => c | 0b00001000,
                    'B' => c | 0b00000100,
                    'R' => c | 0b00000010,
                    'G' => c | 0b00000001,
                    _ => c,
                };
            }
            Ok(c.into())
        }
    }
}

impl ColoursVisitor {
    fn new() -> Self {
        Self
    }
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct RelatedCardObject {
    id: Uuid,
    object: String,
    component: String,
    name: String,
    type_line: String,
    uri: String, // URI
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
    pub image_uris: HashMap<String, String>,
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
    pub fn card_or<'a>(&'a self, query: String) -> SearchResult {
        let c = self.data.first();

        if let Some(card_object) = c {
            if self.total_cards > 1 {
                return SearchResult::MultipleHits(query, self.total_cards, card_object.clone());
            };

            return SearchResult::OneHit(card_object.clone());
        };

        return SearchResult::NoHits(query);
    }
}

#[derive(Debug, Clone)]
pub enum SearchResult {
    MultipleHits(String, u32, CardObject),
    OneHit(CardObject),
    NoHits(String),
}

impl SearchResult {
    #[allow(dead_code)]
    fn get_card_ref<'a>(&'a self) -> Option<&'a CardObject> {
        match self {
            Self::MultipleHits(_, _, card) => Some(card),
            Self::OneHit(card) => Some(card),
            Self::NoHits(_) => None,
        }
    }
}

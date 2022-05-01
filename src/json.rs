use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::fs;

use crate::talent::{Talent};

#[derive(Serialize, Deserialize)]
struct JsonTalent {
    name: String,
    index: usize,
    talent_type: i8,
    max_rank: i8,
    required_invested_points: i8,
    parent_names: Vec<String>,
    children_names: Vec<String>,
    sibling_names: Vec<String>,
}

fn load_json_talents(json_path: &str) -> Result<Vec<JsonTalent>> {
    let file_content = fs::read_to_string(json_path)
        .expect("Couldn't find file.");

    let json_talents: Vec<JsonTalent> = serde_json::from_str(&file_content)?;

    Ok(json_talents)
}

pub fn load_talents(json_path: &str) -> Vec<Talent> {
    let json_talents = load_json_talents(json_path).unwrap();
    let mut talents: Vec<Talent>= Vec::new();
    println!("{} talent information found. Creating Talents.", json_talents.len());
    for talent in json_talents {
        // println!("{}", talent.name);
        talents.push(
            Talent::create(talent.name, talent.index, talent.talent_type, talent.required_invested_points, talent.parent_names, talent.children_names, talent.sibling_names)
        );
    }

    return talents;
}

mod json;
mod talent;

use talent::{Talent};
use json::{load_talents};

fn main() {
    // load json file
    let mut talents = load_talents("src/fake_talents.json");
    Talent::initialize(&mut talents);
}

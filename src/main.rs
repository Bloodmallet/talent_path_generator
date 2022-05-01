mod json;
mod talent;
mod tree;

use talent::{Talent};
use json::{load_talents};
use tree::{Tree};


fn main() {
    // load json file
    let mut talents = load_talents("src/fake_talents.json");
    Talent::initialize(&mut talents);

    let mut tree = Tree::plant(talents);

    // for talent in tree.talents {
    //     if talent.name == String::from("D22") {
    //         println!("{:?}, {:?}", talent.string(), talent.children);
    //     }
    // }

    tree.grow(30);

    // for path in &tree.paths {
    //     println!("{}", tree.string(path));
    // }
}

#[derive(PartialEq)]
pub struct Talent {
    pub name: String,
    pub talent_type: i8,
    pub required_invested_points: i8,
    pub parent_names: Vec<String>,
    pub children_names: Vec<String>,
    pub sibling_names: Vec<String>,

    pub index: usize,
    pub parents: Vec<usize>,
    pub children: Vec<usize>,
    pub siblings: Vec<usize>,
}

fn get_index(talents: &Vec<Talent>, name: &String) -> usize {
    return talents.iter().position(|x| &x.name == name).unwrap();
}

fn get_indizes(talents: &Vec<Talent>, names: &Vec<String>) -> Vec<usize> {
    let mut indizes: Vec<usize> = Vec::new();

    if names.len() == 0 {
        return indizes;
    }

    for index in 0..names.len() {
        let name = &names[index];
        let name_index = get_index(talents, &name);
        indizes.push(name_index);
    }

    return indizes;
}

impl Talent {
    pub fn create(    
        name: String,
        index: usize,
        talent_type: i8,
        required_invested_points: i8,
        parent_names: Vec<String>,
        children_names: Vec<String>,
        sibling_names: Vec<String>,
    ) -> Talent {
        Talent {
            name: name,
            talent_type: talent_type,
            required_invested_points: required_invested_points,
            parent_names: parent_names,
            children_names: children_names,
            sibling_names: sibling_names,

            index: index,
            parents: Vec::new(),
            children: Vec::new(),
            siblings: Vec::new()
        }
    }

    pub fn string(&self) -> String {
        return format!("{name}:{index}(p:{parents:?},c:{children:?},s:{siblings:?})", 
            name=self.name, 
            index=self.index, 
            parents=self.parent_names, 
            children=self.children_names, 
            siblings=self.sibling_names,
        );
    }

    pub fn is_selected(&self, tree: &Vec<bool>) -> bool {
        return tree[self.index];
    }

    pub fn initialize(talents: &mut Vec<Talent>) -> () {
        for index in 0..talents.len() {
            let children: Vec<usize> = get_indizes(talents, &talents[index].children_names);
            let parents: Vec<usize> = get_indizes(talents, &talents[index].parent_names);
            let siblings: Vec<usize> = get_indizes(talents, &talents[index].sibling_names);

            let talent = &mut talents[index];
            talent.parents = parents;
            talent.children =children;
            talent.siblings = siblings
        }
    }

    pub fn select(&self, tree: &Vec<bool>)-> Result<Vec<bool>, &str> {
        if tree[self.index] {
            return Err("Already selected.");
        }

        let mut new_tree = tree.clone();

        new_tree[self.index] = true;

        return Ok(new_tree);
    }
}

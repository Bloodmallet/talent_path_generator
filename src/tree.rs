use crate::talent::{Talent};
use std::time::Instant;

pub struct Tree {
    pub talents: Vec<Talent>,
    pub paths: Vec<Vec<bool>>,
}


impl Tree {
    pub fn plant(talents: Vec<Talent>) -> Tree {
        let mut base_path: Vec<bool> = Vec::new();
        for _ in 0..talents.len() {
            base_path.push(false);
        }

        return Tree {
            talents: talents,
            paths: vec![base_path],
        }
    }

    fn _is_already_selected(&self, indizes: &Vec<usize>, path: &Vec<bool>) -> bool {
        if indizes.len() > 0 {
            let mut selected: Vec<bool> = Vec::new();

            for i in 0..indizes.len() {
                selected.push(self.talents[indizes[i]].is_selected(path));
            }

            if Iterator::any(&mut selected.iter_mut(), |x| x==&true) {
                return true;
            }
        }
        return false;
    }

    fn is_sibling_already_selected(&self, talent: &Talent, path: &Vec<bool>) -> bool {
        return self._is_already_selected(&talent.siblings, path);
    }

    // fn is_parent_already_selected(&self, talent: &Talent, path: &Vec<bool>) -> bool {
    //     return self._is_already_selected(&talent.parents, path);
    // }

    pub fn grow(&mut self, points: i8) -> () {
        if self.paths.len() != 1 {
            panic!("grow expects the presence of exactly one path. {} paths were present", self.paths.len());
        }
        // TODO: Check that existing path is 100% false

        let mut entry_points: Vec<usize> = Vec::new();
        for i in 0..self.talents.len() {
            if self.talents[i].parents.len() == 0 {
                entry_points.push(i);
                if self.talents[i].index != i {
                    panic!("Talent found that doesn't have its matching index");
                }
            }
        }

        let mut existing_paths: Vec<(Vec<bool>, Vec<usize>)> = Vec::new();
        existing_paths.push((self.paths[0].clone(), entry_points));

        for i in 0..points {
            let start_time = Instant::now();
            let mut new_paths : Vec<(Vec<bool>, Vec<usize>)> = Vec::new();

            for i_path in 0..existing_paths.len() {
                let path = &existing_paths[i_path].0;
                let entry_points = &existing_paths[i_path].1;

                // create a path for each entrypoint, if possible, 
                // register new entry points based on the new additional talent.
                for i_entry_point in 0..entry_points.len() {
                    let talent = &self.talents[entry_points[i_entry_point]];
                    let mut new_entry_points = entry_points.clone();

                    if self.is_sibling_already_selected(talent, path) {
                        // nothing to do here
                    } else if talent.is_selected(path) {
                        // nothing to do here
                        // new_entry_points.retain(|&x| x != talent.index);
                    } else if talent.required_invested_points > i {
                        // nothing to do here
                    } else {
                        let new_path = talent.select(path).unwrap();

                        if !new_paths.iter().any(|x| x.0 == new_path) {

                            for i_child in 0..talent.children.len() {
                                let child_id = talent.children[i_child];
                                if ! new_entry_points.contains(&child_id) {
                                    new_entry_points.push(child_id);
                                }
                            }


                            new_entry_points.retain(|&x| x != talent.index);

                            new_paths.push((new_path, new_entry_points));
                        }
                    }
                }
            }

            existing_paths = new_paths;
            println!("{}: {} ({:?})", i+1, existing_paths.len(), start_time.elapsed()); // {:.2?}
        }

        let mut new_paths: Vec<Vec<bool>> = Vec::new();
        for i in 0..existing_paths.len() {
            new_paths.push(existing_paths[i].0.clone());
        }

        self.paths = new_paths;
    }

    pub fn string(&self, path: &Vec<bool>) -> String {
        let mut name = String::new();
        
        for index in 0..path.len() {
            if self.talents[index].is_selected(path) {
                name += &self.talents[index].name;
            }
        }

        return name;
    }
}

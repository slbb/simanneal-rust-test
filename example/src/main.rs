use rand::seq::SliceRandom;
use serde_json;
use simanneal::*;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let file_string = fs::read_to_string("c:/Users/77777/OneDrive/退火脚本/词重码退火/info.json").unwrap();
    let raw_code_list: Vec<String>;
    {
        let mut tmp = Vec::new();
        let info: Vec<(String, i32)> = serde_json::from_str(file_string.as_str()).unwrap();
        for item in info{
            tmp.push(item.0);
        }
        raw_code_list = tmp;
    }
    let mut state: HashMap<char, char> = HashMap::new();
    if let Ok(lines) = read_lines("c:/Users/77777/OneDrive/退火脚本/词重码退火/only_cms2_result.txt") {
        for line in lines {
            if let Ok(line_string) = line {
                let split_items: Vec<&str> = line_string.trim().split("\t").collect();
                let item_a = split_items[0].chars().next().unwrap();
                let item_b = split_items[1].chars().next().unwrap();
                state.insert(item_a, item_b);
            }
        }
    }
    let keys: Vec<char> = state.keys().copied().collect();
    let values_set: HashSet<char> = state.values().copied().collect();
    let values: Vec<char> = values_set.into_iter().collect();
    let mut rng = rand::thread_rng();
    let move_state = |state: &HashMap<char, char>| {
        let key = keys.choose(&mut rng).unwrap();
        let value = values.choose(&mut rng).unwrap();
        let mut result = state.clone();
        result.insert(*key, *value);
        result
    };
    let calc_state_energy = |state: &HashMap<char, char>| {
        let mut code_list_set: HashSet<String> = HashSet::new();
        for s in &raw_code_list {
            let mapping_string: String = s.chars().map(|c|state.get(&c).unwrap()).collect();
            code_list_set.insert(mapping_string);
        }
        (&raw_code_list.len() - code_list_set.len()) as f64
    };
    let (best_state, best_energy) = simanneal(
        state,
        20000,
        move_state,
        calc_state_energy,
        13000_f64,
        default::t(12000_f64, 1_f64),
        2000,
    );
    println!("best energy: {}", best_energy);
    println!("best_state:");
    let mut result: HashMap<char,Vec<char>> = HashMap::new();
    for values in values.iter() {
        result.insert(*values, Vec::new());
    }
    for (k,v) in best_state{
        let vec = result.get_mut(&v).unwrap();
        vec.push(k);
    }
    for (k,v) in result{
        print!("{}: [", k);
        for c in v{
            print!(" {},", c);
        }
        print!("]\n");
    }
}

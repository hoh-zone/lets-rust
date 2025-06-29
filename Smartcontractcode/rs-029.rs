use std::collections::HashMap;

pub fn tally(scores: &Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    
    for (team, score) in scores {
        let val = map.entry(team.to_string()).or_insert(0);
        *val += score;
    }
    map
}
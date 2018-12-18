use std::collections::HashMap;

fn main() {
    let tot_rounds = 71864*100;
    let num_players = 400;

    let mut scores: HashMap<i64, i64> = HashMap::new();
    for i in 0..num_players {
        scores.insert(i, 0);
    }

    let mut arr: Vec<i64> = Vec::new();
    arr.push(0);

    let mut i = 0; // i is the cur marble index
    let mut lowest_available = 1;
    let mut rounds = 0;
    let mut player = 0;

    while rounds < tot_rounds {
        // insert the lower_available
        if lowest_available%23 != 0 {
            i = (i+1)%arr.len();
            arr.insert(i, lowest_available);
        } else {
            i = (i +arr.len()-8)%arr.len() as usize;
            let removed = arr.remove(i);
            let score = scores.entry(player as i64).or_insert(0);
            *score += lowest_available + removed;
        }
        i += 1;
        lowest_available += 1;
        rounds += 1;
        player = (player+1)%num_players;
    }

    let mut max_score = 0;
    for j in 0..num_players {
        let score = scores.get(&j).unwrap();
        if *score > max_score {
            max_score = *score;
        }
    }
    println!("{}", max_score);
}

fn main() {
    let serial_number = 1133;

    let mut power_grid: Vec<Vec<i64>> = Vec::new();
    for y in 1..300+1 {
        let mut row: Vec<i64> = Vec::new();
        for x in 1..300+1 {
            let power_level = compute_power_level(x, y, serial_number);
            row.push(power_level);
        }
        power_grid.push(row);
    }

    let mut results_grid = power_grid.clone();
    let mut best_seen = i64::min_value();
    let mut best_pos = (0, 0, 0);
    for size in 2..300 {
        for y in 0..300-size {
            for x in 0..300-size {
                // update the value in results grid
                for x1 in x..x+size-1 {
                    results_grid[y][x] += power_grid[y+size-1][x1];
                }
                for y1 in y..y+size-1 {
                    results_grid[y][x] += power_grid[y1][x+size-1];
                }
                results_grid[y][x] += power_grid[y+size-1][x+size-1];
                if results_grid[y][x] > best_seen {
                    best_seen = results_grid[y][x];
                    best_pos = (x+1, y+1, size);
                }
            }
        }
        println!("finished checkin size {}", size);
        println!("max so far is {:?}", best_pos);
    }

    println!("best seen {}", best_seen);
    println!("best post {:?}", best_pos);
}

fn compute_power_level(x: i64, y: i64, serial_number: i64) -> i64 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    power_level /= 100;
    power_level = (power_level)%10 - 5;

    return power_level;
}
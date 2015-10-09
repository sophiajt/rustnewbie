fn main() {
    const GRID_SIZE:usize = 21;

    let mut grid:Vec<Vec<u64>> = Vec::new();
    for _ in 0..GRID_SIZE {
        let mut row: Vec<u64> = Vec::new();
        for _ in 0..GRID_SIZE {
            row.push(0);
        }
        grid.push(row);
    }

    //First, put in the paths we know for sure, as there can be only one way to get to the final point
    //from any of the edges (all D or all R)
    for i in 0..GRID_SIZE {
        grid[GRID_SIZE-1][i] = 1;
        grid[i][GRID_SIZE-1] = 1;
    }

    //Next, let's put a cursor that looks for places to fill in
    //A place is ready to fill in if both its D and R have been counted.  Just need to add both together
    let mut done = false;
    while done == false {
        for i in 0..(GRID_SIZE-1) {
            for j in 0..(GRID_SIZE-1) {
                if grid[i+1][j] != 0 && grid[i][j+1] != 0 {
                    grid[i][j] = grid[i+1][j] + grid[i][j+1];

                    if i == 0 && j == 0 { done = true; }
                }
            }
        }
    }

    println!("Answer: {}", grid[0][0]);
}



/*
The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. 
The Elves explain that a previous expedition planted these trees as a reforestation effort. 
Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. 
To do this, you need to count the number of trees that are visible from outside the grid 
when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each 
tree (your puzzle input). For example:

30373
25512
65332
33549
35390

Each tree is represented as a single digit whose value is its height, where 0 is the 
shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter 
than it. Only consider trees in the same row or column; that is, only look up, down, left, 
or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge, 
there are no trees to block the view. In this example, that only leaves the interior nine 
trees to consider:

    The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom 
        since other trees of height 5 are in the way.)
    The top-middle 5 is visible from the top and right.
    The top-right 1 is not visible from any direction; for it to be visible, 
        there would need to only be trees of height 0 between it and an edge.
    The left-middle 5 is visible, but only from the right.
    The center 3 is not visible from any direction; for it to be visible, 
        there would need to be only trees of at most height 2 between it and an edge.
    The right-middle 3 is visible from the right.
    In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

With 16 trees visible on the edge and another 5 visible in the interior, 
    a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?
*/


use std::fs::File;
use std::io::BufRead;

// 2D array of tree heights
type Grid = Vec<Vec<u8>>;

// function to create a grid of zeros of specified dimensions
fn create_grid(n: usize) -> Grid {
    let mut grid = Grid::new();
    for _ in 0..n {
        let mut row = Vec::new();
        for _ in 0..n {
            row.push(0);
        }
        grid.push(row);
    }
    grid
}

// Read the input file into a 2D array of tree heights
fn read_input() -> Grid {
    let mut grid = Grid::new();
    let file = File::open("input").unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        grid.push(row);
    }
    grid
}



// print the grid
fn print_grid(grid: &Grid) {
    // header
    println!("---");
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!("---");
}

// rotate the grid 90 degrees clockwise
// forumula: 
//   A'[i][j] = A[n-1-j][i]
fn rotate_grid(grid: &Grid) -> Grid {
    let mut rotated_grid = Grid::new();
    let n = grid.len();
    for i in 0..n {
        let mut row = Vec::new();
        for j in 0..n {
            row.push(grid[n-1-j][i]);
        }
        rotated_grid.push(row);
    }
    rotated_grid
}

fn rotate_matrix(matrix: &Grid) -> Grid {
    let mut rotated = matrix.clone();
    let n = rotated.len();
    // Transpose the matrix
    for i in 0..n {
        for j in i..n {
            let temp = rotated[i][j];
            rotated[i][j] = rotated[j][i];
            rotated[j][i] = temp;
        }
    }
    // Reverse the rows
    for row in rotated.iter_mut() {
        row.reverse();
    }
    rotated
}

// compute visibility from west
fn find_vis_from_west(grid: &Grid, vis: &mut Grid) {
    let n = grid.len();
    for i in 0..n {
        let mut max = -1;
        for j in 0..n {
            if grid[i][j] as i32 > max {
                vis[i][j] = 1;
                max = grid[i][j] as i32;
            }
        }
    }
}

// compute visibility from all directions
// by rotating the grid (and vis) 3 times
// and accumulating the visibility
fn find_vis(grid: &Grid) -> Grid {
    let mut vis = create_grid(grid.len());
    find_vis_from_west(grid, &mut vis);
    let mut rotated_grid = rotate_grid(grid);
    let mut rotated_vis = rotate_grid(&vis);
    find_vis_from_west(&rotated_grid, &mut rotated_vis);
    rotated_vis = rotate_grid(&rotated_vis);
    rotated_grid = rotate_grid(&rotated_grid);
    find_vis_from_west(&rotated_grid, &mut rotated_vis);
    rotated_vis = rotate_grid(&rotated_vis);
    rotated_grid = rotate_grid(&rotated_grid);
    find_vis_from_west(&rotated_grid, &mut rotated_vis);
    rotated_vis = rotate_grid(&rotated_vis);
    rotated_grid = rotate_grid(&rotated_grid);
    rotated_vis
}







// compute visibility from a particular tree within the grid
// for example in the grid:
//  30373
//  25512
//  65332
//  33549
//  35390
// the tree at (2, 3) can see 1 tree to the north, 2 trees to the east,
// 2 trees to the south, and 1 tree to the west.
// Return the visibility score, which is the product of the number of trees
// seen in each direction (1 * 2 * 2 * 1 = 4)
fn find_vis_from(grid: &Grid, vis: &mut Grid, i: usize, j: usize) -> u32 {
    let n = grid.len();
    let mut score = 1;
    let my_height = grid[i][j];
    let mut dist = 0;
    // north
    for k in (0..i).rev() {
        vis[k][j] = 1;
        dist += 1;
        if grid[k][j] >= my_height {
            break;
        }
    }
    score *= dist;
    dist = 0;
    // east
    for k in j+1..n {
        vis[i][k] = 1;
        dist += 1;
        if grid[i][k] >= my_height {
            break;
        }
    }
    score *= dist;
    dist = 0;
    // south
    for k in i+1..n {
        vis[k][j] = 1;
        dist += 1;
        if grid[k][j] >= my_height {
            break;
        }
    }
    score *= dist;
    dist = 0;
    // west
    for k in (0..j).rev() {
        vis[i][k] = 1;
        dist += 1;
        if grid[i][k] >= my_height {
            break;
        }
    }
    score *= dist;
    score
}

// find the tree with the best visibility score
fn find_best_vis(grid: &Grid) -> (usize, usize, u32) {
    let mut best_i = 0;
    let mut best_j = 0;
    let mut best_score = 0;
    let n = grid.len();
    for i in 0..n {
        for j in 0..n {
            let score = find_vis_from(&grid, &mut create_grid(n), i, j);
            if score > best_score {
                best_i = i;
                best_j = j;
                best_score = score;
            }
        }
    }
    (best_i, best_j, best_score)
}


// sum the visibility
fn sum_vis(vis: &Grid) -> u32 {
    let mut sum = 0;
    for row in vis {
        for c in row {
            sum += *c as u32;
        }
    }
    sum
}

fn main() {
    let grid = read_input();
    print_grid(&grid);

    let mut vis = create_grid(grid.len());
    find_vis_from_west(&grid, &mut vis);
    print_grid(&vis);

    let vis = find_vis(&grid);
    print_grid(&vis);

    let sum = sum_vis(&vis);
    println!("sum = {}", sum);

    let (i, j, score) = find_best_vis(&grid);
    println!("best tree at ({}, {}) with score {}", i, j, score);
}

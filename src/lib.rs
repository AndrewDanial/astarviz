extern crate colored;

use colored::*;
use rand::Rng;
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq)]
enum TileType {
    Empty,
    Start,
    End,
    Path,
}

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq)]
struct GraphNode {
    value: TileType,
    position: Point,
    dst_from_st: i32,
    dst_to_end: i32,
    score: i32,
    //parent: GraphNode,
}
trait Equals {
    fn equals(&self, other: &Self) -> bool;
}

impl Equals for GraphNode {
    fn equals(&self, other: &Self) -> bool {
        if self.position.x == other.position.x && self.position.y == other.position.y {
            return true;
        }
        false
    }
}

pub fn gen_grid() {
    let mut plane: Vec<Vec<GraphNode>> = Vec::new();
    let mut dimensions = Point { x: 10, y: 10 };

    // initialize the grid with empty values
    for i in 0..dimensions.y {
        plane.push(vec![]);
        for j in 0..dimensions.x {
            let curr_node = GraphNode {
                value: TileType::Empty,
                position: Point { x: j, y: i },
                dst_from_st: -1,
                dst_to_end: -1,
                score: -1,
                //parent: None,
            };
            plane[i].push(curr_node);
        }
    }

    //place the starting node
    let (start, x, y) = random_pt(&mut dimensions, TileType::Start);
    plane[y][x] = start;

    // create the ending node, make sure it doesnt overlap with the starting node
    let (mut end, mut new_x, mut new_y) = random_pt(&mut dimensions, TileType::End);
    while new_x == x && new_y == y {
        if dimensions.x == 1 && dimensions.y == 1 {
            break;
        }
        (end, new_x, new_y) = random_pt(&mut dimensions, TileType::End);
    }
    plane[new_y][new_x] = end;

    for i in 0..plane.len() {
        for j in 0..plane[i].len() {
            if plane[i][j].value != TileType::Start {
                plane[i][j].dst_from_st = manhattan_dist(&plane[i][j], &plane[y][x]);
                plane[i][j].dst_to_end = manhattan_dist(&plane[i][j], &plane[new_y][new_x]);
                plane[i][j].score = plane[i][j].dst_from_st + plane[i][j].dst_to_end;
                // the score value F is = distance from start + distance to end
            }
        }
    }
    //display_grid(plane);
    calculate_path(&mut plane, x, y, new_x, new_y);
}

// This is needed to calculate H, which is the manhattan distance, or number of vertical and horizontal moves needed to get from pt1 to pt2

fn calculate_path(plane: &mut Vec<Vec<GraphNode>>, x: usize, y: usize, new_x: usize, new_y: usize) {
    let mut open_list: Vec<GraphNode> = Vec::new();
    let mut closed_list: Vec<GraphNode> = Vec::new();

    open_list.push(plane[y][x]);
    //closed_list.push(plane[y][x]); // add start to closed list

    for i in 0..plane.len() {
        for j in 0..plane[i].len() {
            if plane[i][j].dst_from_st == 1 {
                open_list.push(plane[i][j]); // add all immediately adjacent tiles to the list
            }
        }
    }

    let mut index = lowest_f(&open_list); // get the index of the lowest f score

    let node = open_list[index]; //

    if node.value == TileType::End {
        println!("No path, they were right next to each other lol");
        display_grid(&plane);
        return;
    }
    closed_list.push(node);
    open_list.remove(index); // remove from open

    let list = next_adjacent(&closed_list, &node, plane); // calculate the next adjacents and get their scores
    for i in &list {
        if closed_list.contains(i) {
            continue;
        }
        if !open_list.contains(i) {
            open_list.push(*i);
        }
    }

    while open_list.len() != 0 {
        index = lowest_f(&open_list); // get the index of the lowest f score
        closed_list.push(open_list[index]); // add it to closed

        let node = open_list[index]; //

        if closed_list.contains(&plane[new_y][new_x]) {
            break;
        }
        open_list.remove(index); // remove from open

        let list = next_adjacent(&closed_list, &node, plane); // calculate the next adjacents and get their scores
        for i in &list {
            if closed_list.contains(i) {
                continue;
            }
            if !open_list.contains(i) {
                open_list.push(*i);
            }
        }
    }

    for i in 0..plane.len() {
        for j in 0..plane[i].len() {
            if closed_list.contains(&plane[i][j]) && plane[i][j].value == TileType::Empty {
                plane[i][j].value = TileType::Path;
            }
        }
    }
    display_grid(&plane);
}
fn lowest_f(open_list: &Vec<GraphNode>) -> usize {
    let mut min = open_list[open_list.len() - 1];
    let mut index: usize = open_list.len() - 1;
    for i in (0..open_list.len()).rev() {
        if open_list[i].score < min.score {
            min = open_list[i];
            index = i;
        }
    }
    index
}

fn next_adjacent(
    closed_list: &Vec<GraphNode>,
    node: &GraphNode,
    plane: &Vec<Vec<GraphNode>>,
) -> Vec<GraphNode> {
    let mut adjacents: Vec<GraphNode> = Vec::new();
    for i in 0..plane.len() {
        for j in 0..plane[i].len() {
            let dst = manhattan_dist(&plane[i][j], node);
            if dst == 1 && !closed_list.contains(&plane[i][j]) {
                adjacents.push(plane[i][j]);
            }
        }
    }

    adjacents
}

fn manhattan_dist(pt1: &GraphNode, pt2: &GraphNode) -> i32 {
    let value_x = (pt2.position.x as i32 - pt1.position.x as i32).abs();
    let value_y = (pt2.position.y as i32 - pt1.position.y as i32).abs();
    value_x + value_y
}

fn display_grid(plane: &Vec<Vec<GraphNode>>) {
    for i in 0..plane.len() {
        for j in 0..plane[i].len() {
            let val = plane[i][j].value;
            match val {
                TileType::Empty => print!("{} ", "#".red()),
                TileType::Start => print!("{} ", "X".green()),
                TileType::End => print!("{} ", "O".cyan()),
                TileType::Path => print!("{} ", "-".yellow()),
            }
            /*match val {
                TileType::Empty => print!("{} ", plane[i][j].score),
                TileType::Start => print!("{} ", "X".green()),
                TileType::End => print!("{} ", "O".cyan()),
                TileType::Path => print!("{} ", (plane[i][j].score).to_string().yellow()),
            }*/
        }
        thread::sleep(Duration::from_millis(10));

        println!("");
    }
}

fn random_pt(dimensions: &Point, val: TileType) -> (GraphNode, usize, usize) {
    let mut rng = rand::thread_rng();
    let rand_x = rng.gen_range(0..dimensions.x);
    let rand_y = rng.gen_range(0..dimensions.y);
    let rand_point = Point {
        x: rand_x,
        y: rand_y,
    };

    let starter = GraphNode {
        value: val,
        position: rand_point,
        dst_from_st: -1,
        dst_to_end: -1,
        score: -1,
        //parent: None,
    };

    (starter, rand_x, rand_y)
}

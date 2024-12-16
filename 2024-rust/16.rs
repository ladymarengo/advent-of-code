use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;
use std::usize::MAX;

fn main() {
    let input = read_to_string("input/16.txt").unwrap();
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    '#' => Tile::Wall,
                    _ => Tile::Empty,
                })
                .collect()
        })
        .collect();

    count_score(&map);
}

#[derive(PartialEq)]
enum Tile {
    Start,
    End,
    Wall,
    Empty,
}

#[derive(Copy, Clone, Debug, Hash)]
struct Node {
    score: usize,
    row: usize,
    col: usize,
    direction: Direction,
    f: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f
            .cmp(&other.f)
            .then(self.score.cmp(&other.score))
            .then(self.row.cmp(&other.row))
            .then(self.col.cmp(&other.col))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
            && self.row == other.row
            && self.col == other.col
            && self.direction == other.direction
            && self.score == other.score
    }
}

impl Eq for Node {}

#[derive(PartialEq, Copy, Clone, Debug, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn count_score(map: &Vec<Vec<Tile>>) {
    let mut open: BTreeSet<Node> = BTreeSet::new();
    let mut closed: BTreeSet<Node> = BTreeSet::new();
    let mut best_score = MAX;
    let mut parents: HashMap<SmallNode, (usize, Vec<SmallNode>)> = HashMap::new();
    let mut count_nodes: Vec<SmallNode> = Vec::new();
    open.insert(Node {
        score: 0,
        row: map.len() - 2,
        col: 1,
        direction: Direction::Right,
        f: 0,
    });

    while !open.is_empty() {
        let node = open.pop_first().unwrap();

        let mut successors: Vec<Node> = Vec::new();
        match node.direction {
            Direction::Up => {
                successors.push(Node {
                    score: node.score + 1,
                    row: node.row - 1,
                    col: node.col,
                    direction: Direction::Up,
                    f: 0,
                });
                successors.push(Node {
                    score: node.score + 1001,
                    row: node.row,
                    col: node.col - 1,
                    direction: Direction::Left,
                    f: 0,
                });
                successors.push(Node {
                    score: node.score + 1001,
                    row: node.row,
                    col: node.col + 1,
                    direction: Direction::Right,
                    f: 0,
                });
            }
            Direction::Down => {
                successors.push(Node {
                    score: node.score + 1,
                    row: node.row + 1,
                    col: node.col,
                    direction: Direction::Down,
                    f: 0,
                });
                successors.push(Node {
                    score: node.score + 1001,
                    row: node.row,
                    col: node.col - 1,
                    direction: Direction::Left,
                    f: 0,
                });
                successors.push(Node {
                    score: node.score + 1001,
                    row: node.row,
                    col: node.col + 1,
                    direction: Direction::Right,
                    f: 0,
                });
            }
            Direction::Left => {
                successors.push(Node {
                    score: node.score + 1,
                    row: node.row,
                    col: node.col - 1,
                    direction: Direction::Left,
                    f: 0,
                });
                successors.push(Node {
                    score: node.score + 1001,
                    row: node.row - 1,
                    col: node.col,
                    direction: Direction::Up,
                    f: 0,
                });
                successors.push(Node {
                    score: node.score + 1001,
                    row: node.row + 1,
                    col: node.col,
                    direction: Direction::Down,
                    f: 0,
                });
            }
            Direction::Right => {
                successors.push(Node {
                    score: node.score + 1,
                    row: node.row,
                    col: node.col + 1,
                    direction: Direction::Right,
                    f: 0,
                });
                successors.push(Node {
                    score: node.score + 1001,
                    row: node.row - 1,
                    col: node.col,
                    direction: Direction::Up,
                    f: 0,
                });
                successors.push(Node {
                    score: node.score + 1001,
                    row: node.row + 1,
                    col: node.col,
                    direction: Direction::Down,
                    f: 0,
                });
            }
        };
        for i in 0..3 {
            let successor = &mut successors[i];
            if map[successor.row][successor.col] == Tile::End
                || map[successor.row][successor.col] == Tile::Empty
            {
                let new_node = SmallNode {
                    row: successor.row,
                    col: successor.col,
                    direction: successor.direction,
                };
                let parent_node = SmallNode {
                    row: node.row,
                    col: node.col,
                    direction: node.direction,
                };
                if let Some((node_score, node_parents)) = parents.get_mut(&new_node) {
                    if *node_score == successor.score && !node_parents.contains(&parent_node) {
                        node_parents.push(parent_node);
                    } else if *node_score > successor.score {
                        *node_score = successor.score;
                        node_parents.clear();
                        node_parents.push(parent_node);
                    }
                } else {
                    parents.insert(new_node, (successor.score, Vec::from([parent_node])));
                }
            }
            if map[successor.row][successor.col] == Tile::End {
                best_score = successor.score;
                println!("First answer is {best_score}");
                count_nodes.push(SmallNode {
                    row: successor.row,
                    col: successor.col,
                    direction: successor.direction,
                });
            }
            if map[successor.row][successor.col] == Tile::Empty {
                let potential_score_left =
                    (map.len() - successor.row) + (map[0].len() - successor.col);
                (*successor).f = successor.score + potential_score_left;

                if !open.iter().any(|n| {
                    n.f <= successor.f
                        && n.row == successor.row
                        && n.col == successor.col
                        && n.direction == successor.direction
                }) && !closed.iter().any(|n| {
                    n.f <= successor.f
                        && n.row == successor.row
                        && n.col == successor.col
                        && n.direction == successor.direction
                }) && successor.score <= best_score
                {
                    open.insert(*successor);
                }
            }
        }
        closed.insert(node);
    }

    let mut tiles_set: HashSet<(usize, usize)> = HashSet::new();
    while !count_nodes.is_empty() {
        let node = count_nodes.pop().unwrap();
        tiles_set.insert((node.row, node.col));
        if let Some((_, node_parents)) = parents.get(&node) {
            for parent in node_parents {
                count_nodes.push(*parent);
            }
        }
    }
    println!("Second answer is {}", tiles_set.len());
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct SmallNode {
    row: usize,
    col: usize,
    direction: Direction,
}

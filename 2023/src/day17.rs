use crate::util::point::{DOWN, LEFT, RIGHT, UP};
use crate::util::{grid::Grid, point::Point};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

type State = (Point, Point, u8);

#[derive(Clone, Eq, PartialEq)]
struct SearchNode {
    state: State,
    cost: usize,
    parent: Option<Box<SearchNode>>,
}
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_goal(state: &State, grid: &Grid) -> bool {
    state.0
        == Point(
            (grid.num_rows() - 1) as i32,
            (grid.num_columns() - 1) as i32,
        )
}
fn make_node(state: State, parent: Option<Box<SearchNode>>, grid: &Grid) -> Box<SearchNode> {
    let current_cell_cost = grid
        .at(state.0.0 as usize, state.0.1 as usize)
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;

    let total_cost = if let Some(p) = &parent {
        p.cost + current_cell_cost
    } else {
        current_cell_cost
    };

    Box::new(SearchNode {
        state,
        cost: total_cost,
        parent,
    })
}

fn init() -> Vec<State> {
    let mut init_states = Vec::new();
    init_states.push((Point(0, 1), RIGHT, 1));
    init_states.push((Point(1, 0), DOWN, 1));
    init_states
}

fn succ(state: &State, grid: &Grid) -> Vec<State> {
    let mut successors = Vec::new();
    let (pos, dir, num_steps) = state;
    if *num_steps < 3 {
        successors.push((*pos + *dir, *dir, num_steps + 1));
    }
    if *dir == UP || *dir == DOWN {
        successors.push((*pos + RIGHT, RIGHT, 1));
        successors.push((*pos + LEFT, LEFT, 1));
    } else {
        successors.push((*pos + UP, UP, 1));
        successors.push((*pos + DOWN, DOWN, 1));
    }
    successors
        .into_iter()
        .filter(|(pos, _, _)| grid.in_bounds(pos.0, pos.1))
        .collect()
}

fn succ2(state: &State, grid: &Grid) {
    let mut successors = Vec::new();
    let (pos, dir, num_steps) = state;
    assert!(*num_steps >= 4 && *num_steps <= 10);
    if *num_steps < 10 {
        successors.push((*pos + *dir, *dir, num_steps + 1));
    }
    if *dir == UP || *dir == DOWN {
        successors.push((*pos + RIGHT, RIGHT, 1));
        successors.push((*pos + LEFT, LEFT, 1));
    } else {
        successors.push((*pos + UP, UP, 1));
        successors.push((*pos + DOWN, DOWN, 1));
    }
}

fn uniform_cost_search(grid: &Grid) -> Option<usize> {
    let mut open = BinaryHeap::new();
    for state in init() {
        open.push(make_node(state, None, grid));
    }
    let mut closed = HashSet::new();
    while let Some(node) = open.pop() {
        if closed.insert(node.state) {
            if is_goal(&node.state, grid) {
                return Some(node.cost);
            }
            for next in succ(&node.state, grid) {
                open.push(make_node(next, Some(node.clone()), grid));
            }
        }
    }
    None
}

pub fn part1(input: &str) -> String {
    let grid = Grid::parse(input);
    uniform_cost_search(&grid).unwrap().to_string()
}
pub fn part2(input: &str) -> String {
    input.to_string()
}

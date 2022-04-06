use crate::{ game::Game, Cell, consts::{MAP_SIZE_I, MAP_SIZE_J}};

#[derive(Clone)]
struct PathNode {
    cell: Cell,
    g_cost: i32,
    h_cost: i32,
    f_cost: i32,
    came_from: Box<Option<PathNode>>,
}

const DIRECTIONAL_ARRAY_X: [i32; 4] = [ 0, 1, 0,-1];
const DIRECTIONAL_ARRAY_Y: [i32; 4] = [-1, 0,-1, 0];
const DIST_BETWEEN: i32 = 10;

fn heuristic(a: &Cell, b: &Cell) -> i32 {
    let diff_i: i32 = (a.i as i32 - b.i as i32).abs();
    let diff_j: i32 = (a.j as i32 - b.j as i32).abs();
    ////diff_i.min(diff_j) * DIST_BETWEEN + (diff_i - diff_j).abs() * (DIST_BETWEEN as f32 * DIST_BETWEEN as f32).sqrt() as i32 // Diagonal movement
    diff_i + diff_j
}

pub fn find_path(game: &Game, start_cell: &Cell, end_cell: &Cell) -> Option<Vec<Cell>> {
    let start_node: PathNode = PathNode {
        cell: start_cell.clone(),
        g_cost: 0,
        h_cost: 0,
        f_cost: 0,
        came_from: Box::new(None),
    };
    let end_node: PathNode = PathNode {
        cell: end_cell.clone(),
        g_cost: 0,
        h_cost: 0,
        f_cost: 0,
        came_from: Box::new(None),
    };

    let mut open_set: Vec<PathNode> = Vec::new();
    let mut closed_set: Vec<PathNode> = Vec::new();

    open_set.push(start_node);

    while open_set.len() > 0 {
        let mut lowest_index = 0;
        for i in 0..open_set.len() {
            if open_set[i].f_cost < open_set[lowest_index].f_cost {
                lowest_index = i;
            }
        }
        let current: PathNode = open_set.remove(lowest_index);

        if current.cell == end_node.cell {
            let mut path: Vec<Cell> = Vec::new();
            let mut path_node: &PathNode = &current;
            path.push(current.cell.clone());
            while let Some(previous) = &*path_node.came_from {
                path.push(previous.cell.clone());
                path_node = previous;
            }
            return Some(path);
        }

        // Find neighbors
        for x in 0..4 {
            let i = current.cell.i as i32 + DIRECTIONAL_ARRAY_X[x];
            let j = current.cell.j as i32 + DIRECTIONAL_ARRAY_Y[x];

            if i < 0
               || i >= MAP_SIZE_I as i32
               || j < 0
               || j >= MAP_SIZE_J as i32
               || closed_set.iter().any(|node| node.cell == game.map[j as usize * MAP_SIZE_I + i as usize])
               || !game.map[j as usize * MAP_SIZE_I + i as usize].open_sides[x]
               {
                println!("NOT OPEN, WHEN {} AND {:?}", x, game.map[j as usize * MAP_SIZE_I + i as usize]);
                continue;
            }

            let tentative_g_score = current.g_cost + DIST_BETWEEN; // Orthogonal movement

            if let Some(neighbor) = open_set.iter().find(|&node| node.cell == game.map[j as usize * MAP_SIZE_I + i as usize]) {
                if neighbor.g_cost <= tentative_g_score {
                    continue;
                }
            } else {
                let h_score = heuristic(&game.map[j as usize * MAP_SIZE_I + i as usize], &end_node.cell);
                let neighbor: PathNode = PathNode {
                    cell: game.map[j as usize * MAP_SIZE_I + i as usize].clone(),
                    g_cost: tentative_g_score,
                    h_cost: h_score,
                    f_cost: tentative_g_score + h_score,
                    came_from: Box::new(Some(current.clone())),
                };
                open_set.push(neighbor);
            }
        }
        closed_set.push(current);
    }
    return None;
}

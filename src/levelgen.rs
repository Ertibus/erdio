use rand::{Rng, prelude::{thread_rng, ThreadRng}};
use crate::Cell;

struct Leaf {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    is_leaf: bool,
    left_child: Option<Box<Leaf>>,
    right_child: Option<Box<Leaf>>,
}

impl Leaf {
    fn new(x1: usize, y1: usize, x2: usize, y2:usize) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            is_leaf: true,
            left_child: None,
            right_child: None,
        }
    }
}

pub fn generate_level(
    level_width: usize,
    level_length: usize,
    min_section_size: usize,
    min_room_size: usize,
) -> Vec<Cell> {
    let mut rng = thread_rng();
    let hall_width: usize = 2;

    let mut root: Leaf = Leaf::new(0, 0, level_width, level_length);
    let start_vertical: bool = rng.gen();
    create_rooms(&mut root, start_vertical, min_section_size, hall_width, &mut rng);
    create_rooms(&mut root, start_vertical, min_room_size, 0, &mut rng);
    let mut map = (0..level_length).map(|j| {
        (0..level_width).map(|i| {
            Cell {
                open_sides: [j != 0, i != level_width - 1, j != level_length - 1, i != 0],
                height: rng.gen_range(-0.05..0.05),
                ..Default::default()
            }
        }).collect::<Vec<Cell>>()
    }).flatten().collect::<Vec<Cell>>();
    build_map(&root, &mut map, &mut rng, level_width, level_length);
    return map;
}

fn create_rooms(
    parent: &mut Leaf,
    vertical: bool,
    min_size: usize,
    hall_width: usize,
    rng: &mut ThreadRng,
) {
    if !parent.is_leaf {
        create_rooms(parent.left_child.as_mut().unwrap(), !vertical, min_size, hall_width, rng);
        create_rooms(parent.right_child.as_mut().unwrap(), !vertical, min_size, hall_width, rng);
        return;
    }

    if ((parent.x2 - parent.x1) as f32 / 2.0).floor() < min_size as f32 && vertical
        || ((parent.y2 - parent.y1) as f32 / 2.0).floor() < min_size as f32 && !vertical {
        return;
    }

    let mut left_child;
    let mut right_child;

    if vertical {
        let cut_pos = rng.gen_range((parent.x1 + min_size)..=(parent.x2 - min_size));
        left_child = Leaf::new(parent.x1, parent.y1, cut_pos, parent.y2);
        right_child = Leaf::new(cut_pos + hall_width, parent.y1, parent.x2, parent.y2);
    } else {
        let cut_pos = rng.gen_range((parent.y1 + min_size)..=(parent.y2 - min_size));
        left_child = Leaf::new(parent.x1, parent.y1, parent.x2, cut_pos);
        right_child = Leaf::new(parent.x1, cut_pos + hall_width, parent.x2, parent.y2);
    }

    create_rooms(&mut left_child, !vertical, min_size, hall_width, rng);
    create_rooms(&mut right_child, !vertical, min_size, hall_width, rng);

    parent.is_leaf = false;
    parent.left_child = Some(Box::new(left_child));
    parent.right_child = Some(Box::new(right_child));
}

fn build_map(leaf: &Leaf, map: &mut Vec<Cell>, rng: &mut ThreadRng, map_width: usize, map_length: usize) {
    if !leaf.is_leaf {
        build_map(leaf.left_child.as_ref().unwrap(), map, rng, map_width, map_length);
        build_map(leaf.right_child.as_ref().unwrap(), map, rng, map_width, map_length);
        return;
    }
    let door = rng.gen_range(leaf.x1..leaf.x2);
    for i in (leaf.x1..leaf.x2) {
        if i != door {
            if (map_width * leaf.y2 + i < map_width * map_length) && leaf.y1 != 0 {
                map[map_width * leaf.y1 - map_width + i].open_sides[2] = false;
                map[map_width * leaf.y2 + i].open_sides[0] = false;
            }
            map[map_width * leaf.y1 + i].open_sides[0] = false;
            map[map_width * leaf.y2 - map_width + i].open_sides[2] = false;
            continue;
        }
        if (map_width * leaf.y2 + i < map_width * map_length) && leaf.y1 != 0 {
            map[map_width * leaf.y1 - map_width + i].doors[2] = true;
            map[map_width * leaf.y2 + i].doors[0] = true;
        }
        map[map_width * leaf.y1 + i].doors[0] = true;
        map[map_width * leaf.y2 - map_width + i].doors[2] = true;
    }
    let door = rng.gen_range(leaf.y1..leaf.y2);
    for i in (leaf.y1..leaf.y2) {
        if i != door {
            if map_width * i + leaf.x2 < map_width * map_length && leaf.x1 != 0 {
                map[map_width * i + leaf.x1 - 1].open_sides[1] = false;
                map[map_width * i + leaf.x2].open_sides[3] = false;
            }
            map[map_width * i + leaf.x1].open_sides[3] = false;
            map[map_width * i + leaf.x2 - 1].open_sides[1] = false;
            continue;
        }
        if map_width * i + leaf.x2 < map_width * map_length && leaf.x1 != 0 {
            map[map_width * i + leaf.x1 - 1].doors[1] = true;
            map[map_width * i + leaf.x2].doors[3] = true;
        }
        map[map_width * i + leaf.x1].doors[3] = true;
        map[map_width * i + leaf.x2 - 1].doors[1] = true;
    }
}

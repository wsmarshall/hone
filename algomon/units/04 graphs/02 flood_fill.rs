use std::collections::HashSet;
use std::collections::VecDeque; //for queue
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr; //for Visited

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    r: usize,
    c: usize,
}

fn build_Coordinate(r: i32, c: i32) -> Coordinate {
    Coordinate {
        r: r as usize,
        c: c as usize,
    }
}

fn get_neighbors(r: i32, c: i32, color: i32, image: &Vec<Vec<i32>>) -> Vec<Coordinate> {
    let mut neighbors: Vec<Coordinate> = vec![];
    let deltaRow = [-1, 0, 1, 0];
    let deltaCol = [0, 1, 0, -1];
    for i in 0..deltaRow.len() {
        let neighborRow = r + deltaRow[i];
        let neighborCol = c + deltaCol[i];
        if 0 <= neighborRow
            && neighborRow < image.len().try_into().unwrap()
            && 0 <= neighborCol
            && neighborCol < image[0].len().try_into().unwrap()
        {
            if image[neighborRow as usize][neighborCol as usize] == color {
                neighbors.push(build_Coordinate(neighborRow, neighborCol));
            }
        }
    }

    neighbors
}

fn bfs(image: &mut Vec<Vec<i32>>, point: Coordinate, new_color: i32) {
    let mut queue = VecDeque::new();
    queue.push_back(point.clone());
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let og_color: i32 = image[point.r][point.c];
    image[point.r][point.c] = new_color;
    visited.insert(point);

    while !queue.is_empty() {
        if let Some(coord) = queue.pop_front() {
            let neighbors: Vec<Coordinate> = get_neighbors(
                coord.r.try_into().unwrap(),
                coord.c.try_into().unwrap(),
                og_color,
                &image,
            );
            for n in neighbors {
                if visited.insert(n.clone()) {
                    image[n.r][n.c] = new_color;
                    queue.push_back(n);
                }
            }
        }
    }
}

fn flood_fill(r: i32, c: i32, replacement: i32, image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_image = image.clone();
    bfs(&mut new_image, build_Coordinate(r, c), replacement);

    new_image
}

use std::collections::VecDeque;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr; //for queue

struct Coordinate {
    r: i32,
    c: i32,
}

fn build_Coordinate(r: i32, c: i32) -> Coordinate {
    Coordinate { r, c }
}

fn get_neighbors(r: i32, c: i32, color: i32, image: &Vec<Vec<i32>>) -> Vec<Coordinate> {
    let mut neighbors: Vec<Coordinate> = vec![];
    let deltaRow = (-1, 0, 1, 0);
    let deltaCol = (0, 1, 0, -1);
    for i in 0..deltaRow.len() {
        let neighborRow = r + deltaRow[i];
        let neighborCol = c + deltaCol[i];
        if (0 <= neighborRow
            && neighborRow < image.len()
            && 0 <= neighborCol
            && neighborCol < image[0].len())
        {
            if (image[neighborRow][neighborCol] == color) {
                neighbors.push(build_Coordinate(neighborRow, neighborCol));
            }
        }
    }

    neighbors
}

fn bfs(image: &mut Vec<Vec<i32>>, point: Coordinate, new_color: i32) {}

fn flood_fill(r: i32, c: i32, replacement: i32, image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut replace_list: HashSet<Coordinate> = HashSet::new();
    let mut new_image = image.clone();
    bfs(&mut new_image, build_Coordinate(r, c), replacement);

    new_image
}

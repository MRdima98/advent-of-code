use core::time;
use std::{thread, usize};

pub fn run() {
    let input = include_str!("../input");
    let mut map = vec![];
    let mut starting_pos = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            let num = c.to_digit(10).unwrap() as usize;
            row.push((num, false));
            if num == 0 {
                starting_pos.push((i, j));
            }
        }
        map.push(row);
    }

    let mut sum = 0;
    for start in starting_pos {
        sum += bfs(&mut map.clone(), start);
    }

    print!("The res is: {sum}\n\n");
}

fn bfs(graph: &mut [Vec<(usize, bool)>], coord: (usize, usize)) -> usize {
    let mut directed_edges = vec![];
    directed_edges.push((coord.0, coord.1));
    graph[coord.0][coord.1].1 = true;
    let mut count = 0;
    while !directed_edges.is_empty() {
        let node = directed_edges.pop().unwrap();
        println!("{},{} ", node.0 + 1, node.1 + 1);

        if graph[node.0][node.1].0 == 9 {
            println!("Hit");
            count += 1;
        }

        let neighbour = get_directed_edges(graph, node);
        for ghebur in neighbour {
            if !graph[ghebur.0][ghebur.1].1 {
                graph[ghebur.0][ghebur.1].1 = true;
                directed_edges.push(ghebur);
            }
        }
    }
    //println!("\nCount: {count}");
    count
}

fn get_directed_edges(
    graph: &mut [Vec<(usize, bool)>],
    coord: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut adj = vec![];

    if coord.0 > 0 {
        if graph[coord.0][coord.1].0 + 1 == graph[coord.0 - 1][coord.1].0 {
            adj.push((coord.0 - 1, coord.1));
        }
    }

    if coord.1 > 0 {
        if graph[coord.0][coord.1].0 + 1 == graph[coord.0][coord.1 - 1].0 {
            adj.push((coord.0, coord.1 - 1));
        }
    }

    if coord.0 < graph.len() - 1 {
        if graph[coord.0][coord.1].0 + 1 == graph[coord.0 + 1][coord.1].0 {
            adj.push((coord.0 + 1, coord.1));
        }
    }

    if coord.1 < graph[0].len() - 1 {
        if graph[coord.0][coord.1].0 + 1 == graph[coord.0][coord.1 + 1].0 {
            adj.push((coord.0, coord.1 + 1));
        }
    }

    adj
}

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
        bfs(&mut map, start);
    }

    print!("The res is: {sum}\n\n");
}

fn bfs(graph: &mut [Vec<(usize, bool)>], coord: (usize, usize)) {
    graph[coord.0][coord.1].1 = true;
    let directed_edges = get_directed_edges(graph, coord);
    for edge in directed_edges {
        let node = graph[edge.0][edge.1];

        if node.0 == 9 {
            println!("{}", node.0);
        }

        if !node.1 {
            graph[edge.0][edge.1].1 = true;
            bfs(graph, edge);
        }
    }
}

fn get_directed_edges(
    graph: &mut [Vec<(usize, bool)>],
    coord: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut adj = vec![];
    println!("{},{} ", coord.0 + 1, coord.1 + 1);

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

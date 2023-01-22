use std::collections::BinaryHeap;
use std::env;

extern crate rand;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Edge {
    weight: i32,
    source: usize, // node coords are of the form y_coord*width + x_coord
    target: usize,
}

// union functions on usize vectors
fn find_root (arr: &Vec<usize>, i : usize, ht: usize) -> (usize, usize) {
    if arr[i] == i {
        (i, ht)
    } else {
        find_root(&arr, arr[i], ht+1)
    }
}

fn union(arr: &mut Vec<usize>, i: usize, j:usize) -> bool { // returns true if this connects two components
    let (i_root, i_ht) = find_root(&arr, i, 0);
    let (j_root, j_ht) = find_root(&arr, j, 0);
    if i_root == j_root {
        false
    }
    else {
        if i_ht < j_ht {
            arr[i_root] = j_root;
        } else {
            arr[j_root] = i_root;
        }
        true
    }
}

// Kruskal's Algorithm for minimal spanning trees

fn min_span_tree(nodes: &mut Vec<usize>, edges: &Vec<Edge>, tree: &mut Vec<Edge>) {
    //populates vector passed by reference with a minimal spanning tree
    //  create heap
    let mut heap : BinaryHeap<&Edge> = BinaryHeap::new();
    for e in edges {
        heap.push(e);
    }

    while ! heap.is_empty() {
        let cand = heap.pop().unwrap();
        if union(nodes, cand.source, cand.target) {
            tree.push(*cand);
        }
    }
} 

// create matrix to be printed as maze in passed boolean matrix

fn assemble_maze(matrix : &mut Vec<bool>, tree: &Vec<Edge>, width: usize, height: usize, sp: usize) {

    matrix.clear();
    for _ in 0..sp*sp*width*height {
        matrix.push(false);
    } // fill matrix with 0s, index function is (i,j) -> i + sp*width*j

    for e in tree {
        let index = {
            let x = e.source % width;
            let y = e.source / width;
            sp*x + sp*sp*width*y
        };
        if e.target - e.source == 1 {
            for i in 0..sp + 1 {
                matrix[index + i] = true
            }
        } else {
            for i in 0..sp + 1 {
                matrix[index + sp*width*i] = true
            }
        }
    }
}

fn print_maze(matrix: &Vec<bool>, width: usize, height: usize, sp: usize) {
    let mut maze = String::from("");
    for _ in 0..sp-1 {
        for _ in 0..sp-1 {maze += "#";}
        maze += ".";
        for _ in 0..sp*width - 1 {maze += "#";}
        maze += "\n";
    }

    for j in 0..sp*height - sp + 1 {
        for _ in 1..sp {maze += "#"}
        for i in 0..sp*width {
            if matrix[i + sp*width*j] {
                maze += ".";
            } else {
                maze += "#";
            }
        }
        maze += "\n"; 
    }
    for _ in 0..sp-1 {
        for _ in 0..sp*width - 1 {maze += "#";}
        maze += ".";
        for _ in 0..sp- 1 {maze += "#";}
        maze += "\n";
    }
    println!("{}", maze);
}

// create grid graph of specified width and height with random weights on edges

fn make_graph(width: usize, height: usize, nodes: &mut Vec<usize>, edges: &mut Vec<Edge>) {
    for i in 0..width*height {
        nodes.push(i);
    }
    for j in 0..height-1 {
        for i in 0..width-1 {
            // random negative weights (since max-heap)
            let w1 : i32 = {
                let x : i32 = rand::random();
                if x < 0 {x} else {-x}
            };
            let w2 : i32 = {
                let x : i32 = rand::random();
                if x < 0 {x} else {-x}
            };
            edges.push(Edge {weight: w1, source: i + j*width, target: i + j*width + 1}); // line 1 to the right
            edges.push(Edge {weight: w2, source: i + j*width, target: i + (j+1)*width}) // line 1 down
        }
        let w : i32 = {
            let x : i32 = rand::random();
            if x < 0 {x} else {-x}
        };
        edges.push(Edge {weight: w, source: (j+1)*width - 1, target: (j+2)*width - 1}); // just down for the last one in a row
    }

    for i in 0..width-1 {
            // random negative weight (since max-heap)
            let w1 : i32 = {
                let x : i32 = rand::random();
                if x < 0 {x} else {-x}
            }; 
            edges.push(Edge {weight: w1, source: i + (height-1)*width, target: i + (height-1)*width + 1}); // line 1 to the right
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let width: usize = *&args[0].parse().unwrap();
    let height: usize = *&args[1].parse().unwrap();
    let sp : usize = 3;

    let mut nodes: Vec<usize> = Vec::new();
    let mut edges : Vec<Edge> = Vec::new();
    let mut tree : Vec<Edge> = Vec::new();
    make_graph(width, height, &mut nodes, &mut edges);
    min_span_tree(&mut nodes, &edges, &mut tree);
    let mut maze: Vec<bool> = Vec::new();
    assemble_maze(&mut maze, &tree, width, height, sp);
    print_maze(&mut maze, width, height, sp);
}

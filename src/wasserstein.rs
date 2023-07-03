use crate::graph::{Edge, Graph, Vertex};

pub fn wasserstein_1d(left: Vec<f64>, right: Vec<f64>) -> Result<f64, String> {
    let total_supply: f64 = left.iter().sum();
    let num_vertices = left.len() + right.len();
    let mut supplies = vec![0.0; num_vertices];
    for (i, v) in left.iter().enumerate() {
        supplies[i] = *v;
    }
    for (i, v) in right.iter().enumerate() {
        supplies[i + left.len()] = -1. * *v;
    }
    let mut edges: Vec<Edge> = Vec::new();
    for (i, _) in left.iter().enumerate() {
        for (j, _) in right.iter().enumerate() {
            let mut left_vertex = Vertex::new(i);
            let mut right_vertex = Vertex::new(j + left.len());
            left_vertex.coordinates = (i, 0); // Set coordinates
            right_vertex.coordinates = (j, 0); // Set coordinates
            edges.push(Edge::new(left_vertex, right_vertex));
        }
    }
    let mut graph = Graph::new(num_vertices, total_supply, supplies)?;
    // Adding edges to the graph
    for edge in edges {
        graph.add_edge(edge.left, edge.right, edge.cost, 0)?;
    }

    let total_cost = graph.mcmf()?;
    if total_cost == 0.0 && !are_f64_vecs_equal(&left, &right) {
        panic!("000000.000000 supply{:?}!= Demand {:?}", left, right);
    }

    Ok(total_cost)
}

pub fn wasserstein_1d_sparse(left: Vec<f64>, right: Vec<f64>) -> Result<f64, String> {
    let total_supply: f64 = left.iter().sum();
    let sparse_left: Vec<(usize, f64)> = left
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v != 0.0)
        .map(|(i, &v)| (i, v))
        .collect();
    let sparse_right: Vec<(usize, f64)> = right
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v != 0.0)
        .map(|(i, &v)| (i, -1. * v))
        .collect();
    let num_vertices = sparse_left.len() + sparse_right.len();
    let mut supplies = vec![0.0; num_vertices];
    for (i, (_, v)) in sparse_left.iter().enumerate() {
        supplies[i] = *v;
    }
    for (i, (_, v)) in sparse_right.iter().enumerate() {
        supplies[i + sparse_left.len()] = *v;
    }
    let mut edges: Vec<Edge> = Vec::new();
    for (i, (li, _)) in sparse_left.iter().enumerate() {
        for (j, (rj, _)) in sparse_right.iter().enumerate() {
            let mut left_vertex = Vertex::new(i);
            let mut right_vertex = Vertex::new(j + sparse_left.len());
            left_vertex.coordinates = (*li, 0); // Set coordinates
            right_vertex.coordinates = (*rj, 0); // Set coordinates
            edges.push(Edge::new(left_vertex, right_vertex));
        }
    }
    let mut graph = Graph::new(num_vertices, total_supply, supplies)?;
    // Adding edges to the graph
    for edge in edges {
        graph.add_edge(edge.left, edge.right, edge.cost, 0)?;
    }

    let total_cost = graph.mcmf()?;
    if total_cost == 0.0 && !are_f64_vecs_equal(&left, &right) {
        panic!("0 cost, left: \n{:?}\nright:\n{:?}\n", left, right);
    }
    Ok(total_cost)
}
fn are_f64_vecs_equal(vec1: &[f64], vec2: &[f64]) -> bool {
    vec1.iter()
        .zip(vec2)
        .all(|(a, b)| approx::abs_diff_eq!(a, b, epsilon = f64::EPSILON))
}

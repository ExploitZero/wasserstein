#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Vertex {
    pub index: usize,
    pub coordinates: (usize, usize),
}

impl Vertex {
    pub fn new(index: usize) -> Self {
        Vertex {
            index,
            coordinates: (0, 0),
        }
    }
}

// For now, ground cost across an edge is manhattan distance
pub struct Edge {
    pub left: Vertex,
    pub right: Vertex,
    pub cost: f64,
    pub flow: usize,
}

impl Edge {
    pub fn new(left: Vertex, right: Vertex) -> Self {
        let diff = |l, r| if l < r { r - l } else { l - r };
        let cost = diff(left.coordinates.0, right.coordinates.0) as f64
            + diff(left.coordinates.1, right.coordinates.1) as f64; // Updated to f64
        Edge {
            left,
            right,
            cost,
            flow: 0,
        }
    }
}

pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub max_capacity: f64,
    pub supplies: Vec<f64>,
}
impl Graph {
    pub fn new(num_vertices: usize, max_capacity: f64, supplies: Vec<f64>) -> Result<Self, String> {
        if max_capacity == 0. {
            Err(format!(
                "Need a positive max-capacity on the graph. Got {}",
                max_capacity
            ))
        } else {
            Ok(Graph {
                vertices: (0..num_vertices).map(Vertex::new).collect(),
                edges: Vec::new(),
                max_capacity,
                supplies,
            })
        }
    }

    pub fn add_edge(
        &mut self,
        left: Vertex,
        right: Vertex,
        cost: f64,
        flow: usize,
    ) -> Result<(), String> {
        if left.index >= self.vertices.len() {
            Err(format!(
                "left index {} is out of range {}",
                left.index,
                self.vertices.len()
            ))
        } else if right.index >= self.vertices.len() {
            Err(format!(
                "right index {} is out of range {}",
                right.index,
                self.vertices.len()
            ))
        } else {
            self.edges.push(Edge {
                left,
                right,
                cost,
                flow,
            });
            Ok(())
        }
    }

    // pub fn designate_supply(&mut self, vertex: Vertex, supply: i64) {
    //     self.vertices[vertex.index].supply = supply;
    // }

    // pub fn designate_demand(&mut self, vertex: Vertex, demand: i64) {
    //     self.vertices[vertex.index].supply = -demand;
    // }
    pub fn mcmf(&mut self) -> Result<f64, String> {
        // Modified return type to f64
        let num_vertices = self.vertices.len() as i64;
        let num_edges = self.edges.len() as i64;
        let max_capacity = self.max_capacity;
        // Modified vertex_supplies to Vec<f64>
        let mut vertex_supplies: Vec<f64> = self.supplies.clone();
        let edges_left: Vec<i64> = self.edges.iter().map(|e| e.left.index as i64).collect();
        let edges_right: Vec<i64> = self.edges.iter().map(|e| e.right.index as i64).collect();
        // Modified edge_costs to Vec<f64>
        let edge_costs: Vec<f64> = self.edges.iter().map(|e| e.cost as f64).collect(); // Assuming that Edge's cost is f64 now
        let mut edge_flows: Vec<f64> = vec![0.0; self.edges.len()]; // Modified to Vec<f64>
        let total_cost: f64; // Modified total_cost to f64
        let v_sum = vertex_supplies.iter().sum::<f64>();
        if v_sum > f64::EPSILON {
            vertex_supplies[0] -= v_sum + 1e-15;
        } else if v_sum >= -1e-15 {
            let e15diff = -1e-15 - v_sum.abs();
            vertex_supplies[0] += e15diff;
        }
        // println!("Supplies: {:?}", vertex_supplies.iter().sum::<f64>());
        // println!("Supplies: {:?}", vertex_supplies);
        // println!("edges: {:?}", edges_left);
        // println!("max: {:?}", max_capacity);
        // for (index, edge) in edges_left.iter().enumerate() {
        //     println!(
        //         "Edge from {} to {} has cost {}",
        //         edge, edges_right[index], edge_costs[index]
        //     );
        // }
        // for (index, edge) in edges_left.iter().enumerate() {
        //     println!(
        //         "Edge from {} to {} has flow {}",
        //         edge, edges_right[index], edge_flows[index]
        //     );
        // }

        unsafe {
            total_cost = min_cost_max_flow_double(
                // Modified function call
                num_vertices,
                num_edges,
                max_capacity,
                vertex_supplies.as_ptr(),
                edges_left.as_ptr(),
                edges_right.as_ptr(),
                edge_costs.as_ptr(),
                edge_flows.as_mut_ptr(),
            );
        }
        for (edge, &flow) in self.edges.iter_mut().zip(edge_flows.iter()) {
            if flow < 0.0 {
                return Err(format!(
                    "found negative flow {} on edge {} -> {}",
                    flow, edge.left.index, edge.right.index,
                ));
            } else {
                edge.flow = flow as usize // You may need to handle fractional flows more carefully
            }
        }
        Ok(total_cost)
    }
}

// Modified extern function signature
#[link(name = "wasserstein")]
extern "C" {
    fn min_cost_max_flow_double(
        num_vertices: i64,
        num_edges: i64,
        max_capacity: f64,
        vertex_supplies: *const f64,
        edges_left: *const i64,
        edges_right: *const i64,
        edge_costs: *const f64, // Changed to f64
        edge_flows: *mut f64,
    ) -> f64; // Return type changed to f64
}

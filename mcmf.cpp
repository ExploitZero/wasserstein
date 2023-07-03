#include "lemon/network_simplex.h"
#include "lemon/smart_graph.h"

#include <cstdint>
#include <vector>
#include <cassert>

using namespace lemon;
template<typename IntegerT, typename FloatingT>
FloatingT min_cost_max_flow(
        IntegerT num_vertices,
        IntegerT num_edges,
        FloatingT max_capacity,
        const FloatingT* vertex_supplies,
        const IntegerT* edges_left,
        const IntegerT* edges_right,
        const FloatingT* edge_costs,
        FloatingT* edge_flows
        ) {
    // Check that the vertices are all numbered properly
    for (IntegerT i = 0; i < num_edges; i++) {
        assert(0 <= edges_left[i] && edges_left[i] < num_vertices);
        assert(0 <= edges_right[i] && edges_right[i] < num_vertices);
    }
    // Create a DirectedGraph
    DIGRAPH_TYPEDEFS(SmartDigraph);
    SmartDigraph graph;
    // Populate vertices
    std::vector<Node> vertices;
    for (IntegerT i = 0; i < num_vertices; i++) {
        vertices.push_back(graph.addNode());
    }
    // Populate edges
    std::vector<Arc> edges;
    for (IntegerT i = 0; i < num_edges; i++) {
        edges.push_back(graph.addArc(vertices[edges_left[i]], vertices[edges_right[i]]));
    }
    // Specify supply/demand vertices
    SmartDigraph::NodeMap<FloatingT> supplies(graph);
    for (IntegerT i = 0; i < num_vertices; i++) {
        supplies[vertices[i]] = vertex_supplies[i];
    }
    // Set capacities to max_capacity
    SmartDigraph::ArcMap<FloatingT> capacities(graph, max_capacity);
    // Specify cost of flow through each edge
    SmartDigraph::ArcMap<FloatingT> costs(graph);
    for (IntegerT i = 0; i < num_edges; i++) {
        costs[edges[i]] = edge_costs[i];
    }
    // Set up network simplex object and provide the supply, capacity, and cost maps
    NetworkSimplex<SmartDigraph, FloatingT> network_simplex(graph);
    network_simplex.supplyMap(supplies).upperMap(capacities).costMap(costs);
    // Run the Network Simplex algorithm
    network_simplex.run();
    // Retrieve calculated flow over each edge and populate in array
    for (IntegerT i = 0; i < num_edges; i++) {
        edge_flows[i] = network_simplex.flow(edges[i]);
    }
    // Retrieve and return the total cost of the flow
    return network_simplex.template totalCost<FloatingT>();

}

extern "C" double min_cost_max_flow_double(
    int64_t num_vertices,
    int64_t num_edges,
    double max_capacity,
    const double* vertex_supplies,
    const int64_t* edges_left,
    const int64_t* edges_right,
    const double* edge_costs,
    double* edge_flows
) {
    return min_cost_max_flow<int64_t, double>(
        num_vertices,
        num_edges,
        max_capacity,
        vertex_supplies,
        edges_left,
        edges_right,
        edge_costs,
        edge_flows
    );
}

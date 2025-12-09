#include "day_08.h"

#include <algorithm>
#include <cmath>
#include <fmt/base.h>

#include <fmt/ranges.h>
#include <string>
#include <unordered_set>
#include <vector>

#include "shared/file_utils.h"
#include "shared/string_utils.h"

namespace day_08 {

void run() {
  auto input = shared::read_file("inputs/day_08.txt");
  fmt::print("{}\n", part1(input, 1000));
  fmt::print("{}\n", part2(input));
}

struct Node {
  int x;
  int y;
  int z;

  double euclidean_distance(const Node &other) const {
    return std::hypot(x - other.x, y - other.y, z - other.z);
  }

  bool operator==(const Node &other) const {
    return x == other.x && y == other.y && z == other.z;
  }
};

struct NodeHash {
  std::size_t operator()(const Node &node) const {
    std::size_t h1 = std::hash<int>()(node.x);
    std::size_t h2 = std::hash<int>()(node.y);
    std::size_t h3 = std::hash<int>()(node.z);
    return h1 ^ (h2 << 1) ^ (h3 << 2);
  }
};

struct Connection {
  Node from;
  Node to;
  double distance;

  bool contains(const Node &other) const {
    return from == other || to == other;
  }
};

} // namespace day_08

namespace fmt {
template <> struct formatter<day_08::Node> {
  // Parse the format specifications (e.g., alignment, precision).
  // This example uses default parsing.
  constexpr auto parse(format_parse_context &ctx) { return ctx.begin(); }

  // Format the Node object and write it to the context's output iterator.
  auto format(const day_08::Node &node, format_context &ctx) const {
    // Use fmt::format_to to write the desired string representation
    return fmt::format_to(ctx.out(), "({}-{}-{})", node.x, node.y, node.z);
  }
};
} // namespace fmt

namespace day_08 {

std::vector<Node> parse_input(const std::string &input) {
  auto lines = shared::get_lines(input);
  std::vector<Node> nodes;

  for (const auto &line : lines) {
    auto parts = shared::split_string(line, ",");
    nodes.push_back(
        {std::stoi(parts[0]), std::stoi(parts[1]), std::stoi(parts[2])});
  }

  return nodes;
}

std::vector<Connection> get_sorted_connections(const std::vector<Node> &nodes) {
  std::vector<Connection> connections;

  // generate all possible connections
  for (size_t i = 0; i < nodes.size() - 1; ++i) {
    for (size_t j = i + 1; j < nodes.size(); ++j) {
      auto distance = nodes[i].euclidean_distance(nodes[j]);
      connections.push_back({nodes[i], nodes[j], distance});
    }
  }

  // sort connections by distance
  std::sort(connections.begin(), connections.end(),
            [](const Connection &a, const Connection &b) {
              return a.distance < b.distance;
            });

  return connections;
}

void add_connection(const Connection &connection,
                    std::vector<std::unordered_set<Node, NodeHash>> &circuits) {
  // check if nodes are already in a circuit
  auto it_from = std::find_if(
      circuits.begin(), circuits.end(),
      [connection](auto circuit) { return circuit.contains(connection.from); });

  auto it_to = std::find_if(
      circuits.begin(), circuits.end(),
      [connection](auto circuit) { return circuit.contains(connection.to); });

  if (it_from == circuits.end() && it_to == circuits.end()) {
    // neither node is in a circuit, create a new one
    circuits.push_back(
        {connection.from, connection.to}); // new circuit with both nodes
  } else if (it_from != circuits.end() && it_to != circuits.end() &&
             it_from != it_to) {
    // both nodes are in different circuits, merge them
    if (it_from != it_to) {
      it_from->insert(it_to->begin(), it_to->end());
      circuits.erase(it_to);
    }
  } else if (it_from != circuits.end()) {
    // only 'from' node is in a circuit, add 'to' node
    it_from->insert(connection.to);
  } else if (it_to != circuits.end()) {
    // only 'to' node is in a circuit, add 'from' node
    it_to->insert(connection.from);
  }
}

std::string part1(const std::string &input, int max_connections) {
  auto nodes = parse_input(input);
  std::vector<std::unordered_set<Node, NodeHash>> circuits;
  std::vector<Connection> connections = get_sorted_connections(nodes);

  // build circuits from shortest connections
  int made_connections = 0;
  for (const auto &connection : connections) {
    if (made_connections >= max_connections) {
      break;
    }
    add_connection(connection, circuits);
    made_connections++;
  }

  std::sort(circuits.begin(), circuits.end(),
            [](auto a, auto b) { return a.size() > b.size(); });

  return std::to_string(circuits[0].size() * circuits[1].size() *
                        circuits[2].size());
}

std::string part2(const std::string &input) {

  auto nodes = parse_input(input);
  std::vector<std::unordered_set<Node, NodeHash>> circuits;
  std::vector<Connection> connections = get_sorted_connections(nodes);

  // build circuits from shortest connections
  std::unordered_set<Node, NodeHash> connected_nodes;
  double result;
  for (const auto &connection : connections) {
    add_connection(connection, circuits);
    connected_nodes.insert(connection.from);
    connected_nodes.insert(connection.to);

    if (connected_nodes.size() == nodes.size()) {
      // all nodes are connected
      result = static_cast<double>(connection.to.x) *
               static_cast<double>(connection.from.x);
      break;
    }
  }

  return std::to_string(static_cast<long>(result));
}

} // namespace day_08

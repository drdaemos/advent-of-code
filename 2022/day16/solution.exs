defmodule AdventOfCode.TwentyTwo.Day16 do
    import AdventOfCode.Pathfinding

    @regex ~r/Valve (\w{2}).*=(\d+);.*ves? ((?:,?\s?\w{2})+)/

    def to_graph(line, graph) do
      parts = hd(Regex.scan(@regex, line, capture: :all_but_first))
      flow = Enum.at(parts, 1) |> String.to_integer()
      edges = String.split(Enum.at(parts, 2), ",") |> Enum.map(&String.trim/1)
      Map.put(graph, hd(parts), {flow, edges})
    end
  
    def parse_input(path, default) do
      (with {:ok, text} <- File.read(Path.join(Path.dirname(__ENV__.file), path)), do: text, else: (_ -> default))
      |> String.trim()
      |> String.split("\n")
      |> Enum.reduce(%{}, &to_graph/2)
    end
  
    def dist_g_up(x, y) do
      if y - x > 1, do: 100000, else: 1
    end
    
    def dist_g_down(x, y) do
      if x - y > 1, do: 100000, else: 1
    end

    def find_max_pressure(graph) do
      flowing = Map.filter(graph, fn {_, {flow, edges}} -> flow > 0 end)
    end
  
    @input_test """
    Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II
    """
  
    def part_one(input) do
      graph = parse_input(input, @input_test)
      |> find_max_pressure()
      |> IO.inspect()
      0
    end
  
    # def part_two(input) do
    #   map = parse_input(input, @input_test)
  
    #   goal_fn = fn pos -> map[pos] == 97 end
    #   h_fn = fn _ -> 0 end # disabling heuristic to revert to Dijkstra algo
    #   path = find_path(map, map[:goal], h_fn, &dist_g_down/2, goal_fn)
  
    #   unless is_nil(path), do: Enum.count(path) - 1, else: "No path found"
    # end
  
  
  end
  
  IO.puts("Part one: #{AdventOfCode.TwentyTwo.Day16.part_one("inpsut.txt")}") #
#   IO.puts("Part two: #{AdventOfCode.TwentyTwo.Day16.part_two("input.txt")}") #
  
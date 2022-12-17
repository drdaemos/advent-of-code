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


    def edges(pos, graph) do
      node = Map.get(graph, pos)
      elem(node, 1)
    end

    def chain_size(node, length \\ 0) do
      unless is_nil(node) do
        chain_size(node[:parent], length+1)
      else
        length
      end
    end

    def h_noop(_) do
      0
    end

    def cost_g(_, {flow, _}) do
      if flow != 0, do: 1/flow, else: 1
    end

    def path_cost(path, graph, opened, remaining) do
      trace = walk_path(tl(path), graph, opened)
      flow = Enum.filter(trace, fn {action, _, _} ->
        action == :open
      end)
      |> Enum.map(&elem(&1, 2))
      |> Enum.sum()

      steps = Enum.count(trace)
      cost = flow / :math.pow(steps, 2)
      cost # TODO: fix cost function
    end

    def walk_path(path, graph, opened, trace \\ []) do
      valve = List.first(path)
      if !is_nil(valve) do
        {flow, _} = Map.get(graph, valve)
        added = if valve in opened or flow == 0 or Enum.count(path) > 1, do: [{:move, valve, 0}], else: [{:move, valve, 0}, {:open, valve, flow}]
        trace = trace ++ added
        walk_path(tl(path), graph, opened, trace)
      else
        trace
      end
    end

    def disable_next_valve(graph, current \\ "AA", opened \\ [], remaining) do
      paths = Map.filter(graph, fn {valve, {flow, edges}} -> flow > 0 && !(valve in opened) end)
      |> Enum.map(fn {key, _} ->
        find_path(graph, current, &h_noop/1, &cost_g/2, &(&1 == key), &edges/2, 1000)
      end)
      |> Enum.map(fn path -> Enum.map(path, fn node -> node.pos end) end)
      |> Enum.sort_by(&path_cost(&1, graph, opened, remaining), :desc)

      if Enum.count(paths) > 0 and remaining > 0 do
        List.first(paths)
        |> tl()
        |> walk_path(graph, opened)
      else
        nil
      end
    end

    def sum_pressure(path) do
      Enum.reduce(1..30, {path, 0, 0}, fn _, {remaining, released, releasing} ->
        {_,_,flow} = List.first(remaining, {0,0,0})
        {Enum.drop(remaining, 1), released + releasing, flow + releasing}
      end)
      |> elem(1)
    end

    def find_max_pressure(graph, current \\ "AA", opened \\ [], path \\ [], remaining \\ 30) do
      trace = disable_next_valve(graph, current, opened, remaining)
      unless is_nil(trace) do
        start = List.last(trace) |> elem(1)
        valves = Enum.filter(trace, &(elem(&1, 0) == :open))
        |> Enum.map(&(elem(&1, 1)))
        opened = opened ++ valves
        find_max_pressure(graph, start, opened, path ++ trace, remaining - Enum.count(trace))
      else
        IO.inspect(path)
        sum_pressure(path)
      end
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
      # |> IO.inspect()
      # 0
    end
  end

  IO.puts("Part one: #{AdventOfCode.TwentyTwo.Day16.part_one("input.txt")}") # 2250
  IO.puts("Part one: #{AdventOfCode.TwentyTwo.Day16.part_one("inputs.txt")}") # 2250
#   IO.puts("Part two: #{AdventOfCode.TwentyTwo.Day16.part_two("input.txt")}") #

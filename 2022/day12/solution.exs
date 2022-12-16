defmodule AdventOfCode.TwentyTwo.Day12 do
  import AdventOfCode.Pathfinding

  def fix_start_goal_vals(val) do
    cond do
      val == 83 -> 97
      val == 69 -> 122
      true -> val
    end
  end

  def to_2d_map(list) do
    Enum.reduce(0..(Enum.count(list)-1), %{}, fn y, map ->
      Enum.reduce(0..(Enum.count(Enum.at(list, y))-1), map, fn x, map ->
        val = list |> Enum.at(y) |> Enum.at(x)
        map = Map.put(map, {x, y}, fix_start_goal_vals(val))
        cond do
          val == 83 -> Map.put(map, :start, {x, y})
          val == 69 -> Map.put(map, :goal, {x, y})
          true -> map
        end
      end)
    end)
  end

  def parse_input(path, default) do
    (with {:ok, text} <- File.read(Path.join(Path.dirname(__ENV__.file), path)), do: text, else: (_ -> default))
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&String.to_charlist/1)
    |> to_2d_map()
  end

  def dist_g_up(x, y) do
    if y - x > 1, do: 100000, else: 1
  end
  
  def dist_g_down(x, y) do
    if x - y > 1, do: 100000, else: 1
  end

  def neighbours({x, y}, map) do
    [{0,-1}, {0,1}, {1,0}, {-1,0}]
    |> Enum.reduce([], fn {nx, ny}, nbs ->
      pos = {x + nx, y + ny}
      if Map.has_key?(map, pos), do: nbs ++ [pos], else: nbs
    end)
  end
  
  def dist_pythagorean({x_a, y_a}, {x_b, y_b}) do
    ceil(:math.sqrt(:math.pow(abs(x_b - x_a), 2) + :math.pow(abs(y_b - y_a), 2)))
  end

  @input_test """
  Sabqponm
  abcryxxl
  accszExk
  acctuvwj
  abdefghi
  """

  def part_one(input) do
    map = parse_input(input, @input_test)
    goal_fn = fn pos -> pos == map[:goal] end
    h_fn = fn pos -> dist_pythagorean(pos, map[:goal]) end
    path = find_path(map, map[:start], h_fn, &dist_g_up/2, goal_fn, &neighbours/2)

    unless is_nil(path), do: Enum.count(path) - 1, else: "No path found"
  end

  def part_two(input) do
    map = parse_input(input, @input_test)

    goal_fn = fn pos -> map[pos] == 97 end
    h_fn = fn _ -> 0 end # disabling heuristic to revert to Dijkstra algo
    path = find_path(map, map[:goal], h_fn, &dist_g_down/2, goal_fn, &neighbours/2)

    unless is_nil(path), do: Enum.count(path) - 1, else: "No path found"
  end


end

IO.puts("Part one: #{AdventOfCode.TwentyTwo.Day12.part_one("input.txt")}") #
IO.puts("Part two: #{AdventOfCode.TwentyTwo.Day12.part_two("input.txt")}") #

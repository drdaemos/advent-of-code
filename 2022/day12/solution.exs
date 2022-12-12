defmodule AdventOfCode.TwentyTwo.Day12 do

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

  def neighbours({x, y}, map) do
    [{0,-1}, {0,1}, {1,0}, {-1,0}]
    |> Enum.reduce([], fn {nx, ny}, nbs ->
      pos = {x + nx, y + ny}
      if Map.has_key?(map, pos), do: nbs ++ [pos], else: nbs
    end)
  end

  def dist_g(x, y) do
    if y - x > 1, do: 100000, else: 1
  end

  def dist_h({x_a, y_a}, {x_b, y_b}) do
    ceil(:math.sqrt(:math.pow(abs(x_b - x_a), 2) + :math.pow(abs(y_b - y_a), 2)))
  end

  def to_node(pos, g, h, parent_node \\ nil) do
    parent = if is_nil(parent_node), do: nil, else: %{parent: parent_node.parent, pos: parent_node.pos}
    %{pos: pos, parent: parent, g: g, f: g + h}
  end

  def return_path(current, path \\ []) do
    unless is_nil(current) do
      return_path(current[:parent], path ++ [current])
    else
      Enum.reverse(path)
    end
  end

  def a_star_loop(open, closed, {map, goal}=context, iterations) do
    if Heap.size(open) > 0 and iterations <= 999999 do
      {current, rest_open} = Heap.split(open)
      IO.inspect(Heap.size(rest_open))
      IO.inspect("#{elem(current.pos, 0)}, #{elem(current.pos, 1)} = #{current.f}")
      closed = MapSet.put(closed, current.pos)

      if (current.pos == goal) do
        return_path(current)
      else
        rest_open = neighbours(current.pos, map)
        |> Enum.reduce(rest_open, fn nbr, acc ->
          if nbr in closed do
            acc
          else
            node = to_node(
              nbr,
              current.g + dist_g(map[current.pos], map[nbr]),
              dist_h(nbr, goal),
              current)

            unless Enum.find(acc, fn x -> x.pos == node.pos && x.g < node.g end), do: Heap.push(acc, node), else: acc
          end
        end)

        a_star_loop(rest_open, closed, context, iterations+1)
      end
    else
      IO.inspect(iterations)
      # IO.inspect(open)
      IO.inspect(closed)
      nil
    end
  end

  def find_path(map, start, goal) do
    ordering = fn x, y -> x.f < y.f end
    open = Heap.new(ordering) |> Heap.push(to_node(start, 0, 0))
    closed = MapSet.new()

    a_star_loop(open, closed, {map, goal}, 0)
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
    IO.inspect(map)
    path = find_path(map, map[:start], map[:goal])

    unless is_nil(path), do: Enum.count(path) - 1, else: "No path found"
  end

  def part_two(input) do
    parse_input(input, @input_test)
    # |> apply_rules(List.duplicate({0, 0}, 10))
  end

end

IO.puts("Part one: #{AdventOfCode.TwentyTwo.Day12.part_one("input.txt")}") #
# IO.puts("Part two: #{AdventOfCode.TwentyTwo.Day12.part_two("input.txt")}") #

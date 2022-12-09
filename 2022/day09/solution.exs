defmodule Solution do
  @directions %{"U" => :up, "R" => :right, "L" => :left, "D" => :down}

  def parse_input(path) do
    File.read!(path)
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&parse_rule/1)
  end

  def parse_rule(str) do
    str
    |> String.split()
    |> List.update_at(0, fn val -> @directions[val] end)
    |> List.update_at(1, &String.to_integer/1)
    |> List.to_tuple()
  end

  def move_head({x, y}, dir) do
    case dir do
      :up -> {x, y + 1}
      :down -> {x, y - 1}
      :right -> {x + 1, y}
      :left -> {x - 1, y}
    end
  end

  def distance({x_a, y_a}, {x_b, y_b}) do
    :math.sqrt(:math.pow(abs(x_b - x_a), 2) + :math.pow(abs(y_b - y_a), 2))
  end

  def drag_knot({x_a, y_a}, {x_b, y_b}) do
    cond do
      (x_a == x_b) and (y_a > y_b) -> {x_b, y_b + 1}    # up
      (x_a > x_b) and (y_a == y_b) -> {x_b + 1, y_b}    # right
      (x_a == x_b) and (y_a < y_b) -> {x_b, y_b - 1}    # down
      (x_a < x_b) and (y_a == y_b) -> {x_b - 1, y_b}    # left
      (x_a > x_b) and (y_a > y_b) -> {x_b + 1, y_b + 1} # up-right
      (x_a > x_b) and (y_a < y_b) -> {x_b + 1, y_b - 1} # down-right
      (x_a < x_b) and (y_a < y_b) -> {x_b - 1, y_b - 1} # down-left
      (x_a < x_b) and (y_a > y_b) -> {x_b - 1, y_b + 1} # up-left
      true -> {x_b, y_b}
    end
  end

  def move_rope(rope, dir) do
    rope
    |> Enum.reduce([], fn knot, rope ->
      if Enum.empty?(rope) do
        [move_head(knot, dir)]
      else
        prev = List.first(rope)
        dist = distance(prev, knot)
        knot = cond do
          dist >= 2 -> drag_knot(prev, knot)
          true -> knot
        end
        [knot | rope] # appending because it is faster
      end
    end)
    |> Enum.reverse() # reversing rope
  end

  def apply_rule({dir, repeats}, {rope, set}) do
    if repeats > 0 do
      rope = move_rope(rope, dir)
      apply_rule({dir, repeats - 1}, {rope, MapSet.put(set, List.last(rope))})
    else
      {rope, set}
    end
  end

  def apply_rules(rules, rope) do
    set = MapSet.new([List.last(rope)])

    rules
    |> Enum.reduce({rope, set}, &apply_rule/2)
    |> elem(1)
    |> Enum.count()
  end

  def part_one(input) do
    parse_input(input)
    |> apply_rules(List.duplicate({0, 0}, 2))
  end

  def part_two(input) do
    parse_input(input)
    |> apply_rules(List.duplicate({0, 0}, 10))
  end

end

IO.puts("Part one: #{Solution.part_one("input.txt")}") # 6498
IO.puts("Part two: #{Solution.part_two("input.txt")}") # 2531

defmodule Solution do
  @directions %{"U" => :up, "R" => :right, "L" => :left, "D" => :down}

  def parse_input(path, default) do
    (with {:ok, text} <- File.read(path), do: text, else: (_ -> default))
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

  def move_knot({x, y}, dir) do
    case dir do
      :up -> {x, y + 1}
      :down -> {x, y - 1}
      :left -> {x - 1, y}
      :right -> {x + 1, y}
    end
  end

  def distance({x_a, y_a}, {x_b, y_b}) do
    :math.sqrt(:math.pow(abs(x_b - x_a), 2) + :math.pow(abs(y_b - y_a), 2))
  end

  def move_if(knot, compfn, dir) do
    if compfn.(), do: move_knot(knot, dir), else: knot
  end

  def drag_knot({x_a, y_a}, {x_b, y_b}) do
    {x_b, y_b}
    |> move_if(fn -> y_a > y_b end, :up)
    |> move_if(fn -> y_a < y_b end, :down)
    |> move_if(fn -> x_a < x_b end, :left)
    |> move_if(fn -> x_a > x_b end, :right)
  end

  def move_rope(rope, dir) do
    rope
    |> Enum.reduce([], fn knot, rope ->
      if Enum.empty?(rope) do
        [move_knot(knot, dir)]
      else
        prev = List.first(rope)
        knot = if distance(prev, knot) >= 2, do: drag_knot(prev, knot), else: knot
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

  @input_test """
  R 5
  U 8
  L 8
  D 3
  R 17
  D 10
  L 25
  U 20
  """

  def part_one(input) do
    parse_input(input, @input_test)
    |> apply_rules(List.duplicate({0, 0}, 2))
  end

  def part_two(input) do
    parse_input(input, @input_test)
    |> apply_rules(List.duplicate({0, 0}, 10))
  end

end

IO.puts("Part one: #{Solution.part_one("input.txt")}") # 6498
IO.puts("Part two: #{Solution.part_two("input.txt")}") # 2531

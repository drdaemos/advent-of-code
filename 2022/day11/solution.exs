defmodule Solution do

  def parse_input(path) do
    File.read!(path)
    |> String.trim()
    |> String.split("\n\n")
    |> Enum.map(&parse_record/1)
  end

  def insert_into(val, key, record) do
    Map.put(record, key, val)
  end

  def get_last_int(str) do
    str
    |> String.split()
    |> List.last()
    |> String.to_integer()
  end

  def parse_line(line, record) do
    cond do
      String.starts_with?(line, "Starting items:") ->
        line
        |> String.split(":")
        |> List.last()
        |> String.split(",")
        |> Enum.map(&String.trim/1)
        |> Enum.map(&String.to_integer/1)
        |> insert_into(:items, record)
      String.starts_with?(line, "Operation:") ->
        line
        |> String.split(":")
        |> List.last()
        |> String.trim()
        |> insert_into(:op, record)
      String.starts_with?(line, "Test: divisible by") ->
        get_last_int(line)
        |> insert_into(:test, record)
      String.starts_with?(line, "If true: throw to monkey") ->
        get_last_int(line)
        |> insert_into(:iftrue, record)
      String.starts_with?(line, "If false: throw to monkey") ->
        get_last_int(line)
        |> insert_into(:iffalse, record)
      true -> record
    end
  end

  def parse_record(str) do
    str
    |> String.split("\n")
    |> Enum.map(&String.trim/1)
    |> Enum.reduce(%{}, &parse_line/2)
  end

  def inspect_and_throw(index, {monkeys, inspects}) do
    monkey = monkeys
    |> Enum.at(index)

    items = monkey.items

    if Enum.count(items) > 0 do
      item = hd(items)

      level = Code.eval_string(monkey.op, [old: item])
      |> elem(0)
      # |> then(fn x -> floor(x / 3) end)

      to_idx = if rem(level, monkey.test) == 0,
        do: monkey.iftrue,
        else: monkey.iffalse

      new = monkeys
      |> List.update_at(index, fn rec -> Map.update!(rec, :items, &Enum.drop(&1, 1)) end)
      |> List.update_at(to_idx, fn rec -> Map.update!(rec, :items, fn list -> list ++ [level] end) end)

      inspect_and_throw(index, {new, Map.update(inspects, index, 1, &(&1 + 1))})
    else
      {monkeys, inspects}
    end
  end

  def play_round(_round, {monkeys, inspects}) do
    (0..(Enum.count(monkeys)-1))
    |> Enum.reduce({monkeys, inspects}, &inspect_and_throw/2)
  end

  def part_one(input) do
    monkeys = input
    |> parse_input()

    (1..1000)
    |> Enum.reduce({monkeys, %{}}, &play_round/2)
    |> elem(1)
    |> Map.values()
    |> Enum.sort()
    |> Enum.reverse()
    |> Enum.take(2)
    |> Enum.product()
    |> IO.inspect(charlists: :as_lists)
  end

  def part_two(input) do
    input
    0
  end

end

IO.puts ("Part one: #{Solution.part_one("input_test.txt")}") # 111210
# IO.puts ("Part two: ")
# IO.puts Solution.part_two("input.txt") # EHPZPJGL

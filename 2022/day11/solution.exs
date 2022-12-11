defmodule AdventOfCode.TwentyTwo.Day11 do
  def parse_input(path, default) do
    (with {:ok, text} <- File.read(path), do: text, else: (_ -> File.read!(default)))
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
        |> String.split("=")
        |> List.last()
        |> String.split()
        |> Enum.drop(1)
        |> List.to_tuple()
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

  def apply_math(item, {command, arg}) do
    arg = if arg == "old", do: item, else: String.to_integer(arg)
    case command do
      "*" -> item * arg
      "+" -> item + arg
    end
  end

  def inspect_and_throw(index, monkeys, worry_fn) do
    monkey = monkeys
    |> Enum.at(index)

    if Enum.count(monkey.items) > 0 do
      item = hd(monkey.items)

      level = item
      |> apply_math(monkey.op)
      |> worry_fn.()

      to_idx = if rem(level, monkey.test) == 0,
        do: monkey.iftrue,
        else: monkey.iffalse

      updated = monkeys
      |> List.update_at(index, fn rec -> Map.update(rec, :inspects, 1, &(&1 + 1)) end)
      |> List.update_at(index, fn rec -> Map.update!(rec, :items, &Enum.drop(&1, 1)) end)
      |> List.update_at(to_idx, fn rec -> Map.update!(rec, :items, &(&1 ++ [level])) end)

      inspect_and_throw(index, updated, worry_fn)
    else
      monkeys
    end
  end

  def play_round(monkeys, worry_fn) do
    (0..(Enum.count(monkeys)-1))
    |> Enum.reduce(monkeys, &inspect_and_throw(&1, &2, worry_fn))
  end

  def count_business(monkeys) do
    monkeys
    |> Enum.map(&Map.get(&1, :inspects))
    |> Enum.sort()
    |> Enum.reverse()
    |> Enum.take(2)
    |> Enum.product()
  end

  def part_one(input) do
    monkeys = parse_input(input, "input_test.txt")

    (1..20)
    |> Enum.reduce(monkeys, fn _, acc -> play_round(acc, &(floor(&1 / 3))) end)
    |> count_business()
  end

  def part_two(input) do
    monkeys = parse_input(input, "input_test.txt")

    lcm = monkeys
    |> Enum.map(&Map.get(&1, :test))
    |> Enum.product()

    (1..10000)
    |> Enum.reduce(monkeys, fn _, acc -> play_round(acc, &(rem(&1, lcm))) end)
    |> count_business()
  end
end

IO.puts ("Part one: #{AdventOfCode.TwentyTwo.Day11.part_one("input.txt")}") # 111210
IO.puts ("Part two: #{AdventOfCode.TwentyTwo.Day11.part_two("input.txt")}") # 15447387620

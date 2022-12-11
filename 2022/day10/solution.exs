defmodule Solution do
  @command %{"noop" => :noop, "addx" => :addx}

  def parse_input(path, default) do
    (with {:ok, text} <- File.read(path), do: text, else: (_ -> File.read!(default)))
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&parse_statement/1)
  end

  def parse_statement(str) do
    str
    |> String.split()
    |> List.update_at(0, fn val -> @command[val] end)
    |> List.update_at(1, &String.to_integer/1)
    |> List.to_tuple()
  end

  def peek_delay(stmt_index, program) do
    stmt = Enum.at(program, stmt_index, {:noop})
    case elem(stmt, 0) do
      :noop -> 0
      :addx -> 1
    end
  end

  def apply_stmt(_cycle, {register, stmt_index, delay}, program) do
    stmt = Enum.at(program, stmt_index, {:unknown})
    next_delay = peek_delay(stmt_index + 1, program)
    case elem(stmt, 0) do
      :noop -> {register, stmt_index + 1, next_delay}
      :addx -> if delay > 0, do: {register, stmt_index, delay - 1}, else: {register + elem(stmt, 1), stmt_index + 1, next_delay}
    end
  end

  def execute_program(program) do
    register = 1
    Enum.to_list(1..240)
    |> Enum.scan({register, 0, peek_delay(0, program)}, &apply_stmt(&1, &2, program))
    |> Enum.map(&elem(&1, 0))
    |> then(fn x -> [register | x] end)
  end

  def part_one(input) do
    parse_input(input, "input_test.txt")
    |> execute_program()
    |> Enum.slice(19, 220)
    |> Enum.take_every(40)
    |> Enum.zip(20..220//40)
    |> Enum.map(&Tuple.product/1)
    |> Enum.sum()
  end

  def crt_draw({register, cycle}, screen) do
    mod = rem(cycle, 40)
    pixel = if abs(register - mod + 1) <= 1, do: "█", else: " "
    newline = if mod == 0, do: "\n", else: ""
    screen <> pixel <> newline
  end

  def part_two(input) do
    parse_input(input, "input_test.txt")
    |> execute_program()
    |> Enum.zip(1..240)
    |> Enum.reduce("", &crt_draw/2)
  end

end

IO.puts ("Part one: #{Solution.part_one("input.txt")}") # 12520
IO.puts ("Part two: ")
IO.puts Solution.part_two("input.txt") # EHPZPJGL

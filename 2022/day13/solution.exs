defmodule AdventOfCode.TwentyTwo.Day13 do
  
    def parse_input(path, default) do
      (with {:ok, text} <- File.read(Path.join(Path.dirname(__ENV__.file), path)), do: text, else: (_ -> default))
      |> String.trim()
      |> String.split("\n\n")
      |> Enum.map(&String.split(&1, "\n"))
      |> Enum.map(fn pair -> Enum.map(pair, &parse_packet/1) end)
    end

    def parse_packet(str) when str == [] do
      str
    end

    def parse_packet(str) when str != "" do
      String.slice(str, 1, String.length(str)-2)
      |> parse_values()
      |> Enum.map(fn token -> 
        case Integer.parse(token) do
          :error -> parse_packet(token)
          {int, _} -> int
        end
      end)
    end

    def parse_values(str) do
      String.graphemes(str)
      |> Enum.reduce({[], "", 0}, fn char, {list, current, depth} ->
        case char do
          "," -> if depth == 0, do: {list ++ [current], "", depth}, else: {list, current <> char, depth}
          "[" -> {list, current <> char, depth + 1}
          "]" -> {list, current <> char, depth - 1}
          _   -> {list, current <> char, depth}
        end
      end)
      |> then(fn {list, current, _} -> if current != "", do: list ++ [current], else: list end)
    end

    def compare_pair(left, right) do
      lef = List.first(left)
      rig = List.first(right)

      decision = cond do
        is_nil(lef) and is_nil(rig) -> :unknown
        is_nil(lef) -> true
        is_nil(rig) -> false
        is_list(lef) and is_list(rig) -> compare_pair(lef, rig)
        is_list(lef) and is_integer(rig) -> compare_pair(lef, [rig])
        is_integer(lef) and is_list(rig) -> compare_pair([lef], rig)
        left < right -> true
        left > right -> false
        true -> :unknown
      end

      if decision != :unknown or (Enum.count(right) < 2 and Enum.count(left) < 2) do 
        decision
      else 
        compare_pair(Enum.drop(left, 1), Enum.drop(right, 1))
      end
    end

    @input_test """
    [1,1,3,1,1]
    [1,1,5,1,1]
    
    [[1],[2,3,4]]
    [[1],4]
    
    [9]
    [[8,7,6]]
    
    [[4,4],4,4]
    [[4,4],4,4,4]
    
    [7,7,7,7]
    [7,7,7]
    
    []
    [3]
    
    [[[]]]
    [[]]
    
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]
    """
  
    def part_one(input) do
      parse_input(input, @input_test)
      |> Enum.with_index(1)
      |> Enum.filter(fn {pair, _} -> apply(&compare_pair/2, pair) end)
      |> Enum.map(&elem(&1, 1))
      |> Enum.sum()
      
    end
  
    def part_two(input) do
      dividers = [[[2]],[[6]]]
      parse_input(input, @input_test)
      |> Enum.concat()
      |> Enum.concat(dividers)
      |> Enum.sort(fn a, b -> if a == b, do: true, else: compare_pair(a, b) end)
      |> Enum.with_index(1)
      |> Enum.filter(fn {packet, _} -> packet in dividers end)
      |> Enum.map(&elem(&1, 1))
      |> Enum.product()
    end
  end
  
  IO.puts("Part one: #{AdventOfCode.TwentyTwo.Day13.part_one("input.txt")}") # 6187
  IO.puts("Part two: #{AdventOfCode.TwentyTwo.Day13.part_two("input.txt")}") # 23520
  
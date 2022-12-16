defmodule AdventOfCode.Pathfinding do
  
  def find_path(map, start, h_fn, g_fn, goal_fn, neighbours_fn, max_iterations \\ nil) do
    ordering = fn x, y -> x.f < y.f end
    open = Heap.new(ordering) |> Heap.push(to_node(start, 0, 0))
    closed = MapSet.new()
    max_iterations = unless is_nil(max_iterations), do: max_iterations, else: Enum.count(map)
    
    a_star_loop(open, closed, {map, h_fn, g_fn, goal_fn, neighbours_fn, max_iterations}, 0)
  end
  
  def a_star_loop(open, closed, {map, h_fn, g_fn, goal_fn, neighbours_fn, max_iterations}=context, iterations) do
    if Heap.size(open) > 0 and iterations <= max_iterations do
      {current, rest_open} = Heap.split(open)
      closed = MapSet.put(closed, current.pos)
      
      if goal_fn.(current.pos) do
        return_path(current)
      else
        updated = Enum.reduce(neighbours_fn.(current.pos, map), rest_open, fn nbr, acc ->
          if nbr in closed do
            acc
          else
            node = to_node(
            nbr,
            current.g + g_fn.(map[current.pos], map[nbr]),
            h_fn.(nbr),
            current)
            
            unless Enum.find(acc, fn x -> x.pos == node.pos && x.g <= node.g end), do: Heap.push(acc, node), else: acc
          end
        end)
        
        a_star_loop(updated, closed, context, iterations+1)
      end
    else
      nil
    end
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
end
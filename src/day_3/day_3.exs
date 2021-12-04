defmodule Day3 do
  @moduledoc """
  [Day 3](https://adventofcode.com/2021/day/3): Submarine power consumption.
  """

  defp to_int(list), do: Enum.join(list) |> String.to_integer(2)

  def power_consumption_1(input) do
    input
    |> Stream.map(&String.graphemes/1)
    |> Stream.map(fn line -> Stream.map(line, &String.to_integer/1) end)
    |> Stream.zip()
    |> Stream.map(fn col ->
      Tuple.to_list(col) |> Enum.frequencies() |> Enum.max_by(&elem(&1, 1)) |> elem(0)
    end)
    |> Stream.map(fn
      0 -> {0, 1}
      1 -> {1, 0}
    end)
    |> Enum.unzip()
    |> then(fn {gamma, epsilon} -> to_int(gamma) * to_int(epsilon) end)
  end

  defp rating(input, min_max, idx \\ 0) do
    input
    |> Enum.group_by(&String.at(&1, idx))
    |> then(fn map ->
      case min_max do
        :min -> Enum.min_by(map, &length(elem(&1, 1)))
        :max -> Enum.max_by(map, &length(elem(&1, 1)), &>/2)
      end
    end)
    |> elem(1)
    |> then(fn
      [res] -> String.to_integer(res, 2)
      res -> rating(res, min_max, idx + 1)
    end)
  end

  def power_consumption_2(input), do: rating(input, :min) * rating(input, :max)
end

input =
  File.stream!("input.txt")
  |> Stream.map(&String.trim/1)

IO.puts(Day3.power_consumption_1(input))
# 2250414
IO.puts(Day3.power_consumption_2(input))
# 6085575

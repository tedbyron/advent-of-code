defmodule Day2 do
  @moduledoc """
  [Day 2](https://adventofcode.com/2020/day/2): Submarine movement.
  """

  @typep commands() :: [{String.t(), pos_integer()}]

  @spec dive_1(commands()) :: integer()
  def dive_1(commands) do
    commands
    |> Enum.reduce({0, 0}, fn
      {"forward", distance}, {x, y} -> {x + distance, y}
      {"down", distance}, {x, y} -> {x, y + distance}
      {"up", distance}, {x, y} -> {x, y - distance}
    end)
    |> Tuple.product()
  end

  @spec dive_2(commands()) :: integer()
  def dive_2(commands) do
    commands
    |> Enum.reduce({0, 0, 0}, fn
      {"forward", distance}, {aim, x, y} -> {aim, x + distance, y + aim * distance}
      {"down", distance}, {aim, x, y} -> {aim + distance, x, y}
      {"up", distance}, {aim, x, y} -> {aim - distance, x, y}
    end)
    |> Tuple.delete_at(0)
    |> Tuple.product()
  end
end

input =
  File.stream!("input.txt")
  |> Stream.map(&String.split/1)
  |> Stream.map(fn [direction, distance] -> {direction, String.to_integer(distance)} end)

IO.puts(Day2.dive_1(input))
# 1694130
IO.puts(Day2.dive_2(input))
# 1698850445

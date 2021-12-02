defmodule Day1 do
  @moduledoc """
  [Day 1](https://adventofcode.com/2020/day/1): Sonar sweep thingy.
  """

  @type depths() :: Enum.t()

  @spec sonar_sweep_1(depths()) :: integer()
  def sonar_sweep_1(depths) do
    depths
    |> Stream.chunk_every(2, 1, :discard)
    |> Stream.filter(fn [a, b] -> a < b end)
    |> Enum.count()
  end

  # @spec sonar_sweep_2(depths()) :: integer()
  # def sonar_sweep_2(depths) do
  # end
end

input =
  File.stream!("input.txt", [])
  |> Stream.map(&String.trim_trailing/1)
  |> Stream.map(&String.to_integer/1)

IO.puts(Day1.sonar_sweep_1(input))
# IO.puts(Day1.sonar_sweep_2(input))

defmodule Day1 do
  @moduledoc """
  [Day 1](https://adventofcode.com/2020/day/1): Sonar sweep thingy.
  """

  @typedoc """
  List of depths.
  """
  @type depths() :: Enum.t()

  @doc """
  Count number of times that a depth increases from the previous measurement.
  """
  @spec sonar_sweep_1(depths()) :: integer()
  def sonar_sweep_1(depths) do
    depths
    |> Stream.map(&String.trim_trailing/1)
    |> Stream.map(&String.to_integer/1)
    |> Enum.chunk_every(2, 1)
    |> IO.inspect()

    # |> Enum.filter(fn [a | b] -> a < b end)
  end

  @doc """
  Count the number of times the sum of a 3-measurement window increases from the previous sum of a
  3-measurement window.
  """
  # @spec sonar_sweep_2(depths()) :: integer()
  # def sonar_sweep_2(depths) do
  # end
end

input = File.stream!(Path.join(File.cwd!(), "input.txt"), [])
Day1.sonar_sweep_1(input)

defmodule AdventOfCode.MixProject do
  use Mix.Project

  def project do
    [
      app: :advent_of_code,
      version: "1.0.0",
      deps: deps()
    ]
  end

  # Run "mix help deps" to learn about dependencies
  defp deps do
    [
      {:heap, "~> 2.0"}
    ]
  end
end

defmodule AstcEncoder.MixProject do
  use Mix.Project

  def project do
    [
      app: :astc_encoder,
      version: "0.1.0",
      elixir: "~> 1.16",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
      {:exsync, "~> 0.4.1", only: :dev, runtime: false},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:vix, "~> 0.29.0", only: [:dev, :test], runtime: false},
      {:rustler, "~> 0.34.0", runtime: false}
    ]
  end
end

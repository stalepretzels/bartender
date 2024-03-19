defmodule Bartender.Mixfile do
    use Mix.Project
  
    def project do
      [
        app: :nif_bartender,
        version: "0.1.0",
        elixir: "~> 1.11",
        start_permanent: Mix.env() == :prod,
        deps: deps(),
        aliases: aliases()
      ]
    end
  
    def application do
      [
        extra_applications: [:logger]
      ]
    end
  
    defp deps do
      [
        {:rustler, "~> 0.31.0"}
      ]
    end
  
    defp aliases do
      [
        fmt: [
          "format",
          "cmd cargo fmt --manifest-path native/bartender/Cargo.toml"
        ]
      ]
    end
  end
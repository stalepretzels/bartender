defmodule Bartender.Mixfile do
    use Mix.Project
  
    def project do
      [
        app: :nif_bartender,
        version: "0.1.0",
        elixir: "~> 1.11",
        start_permanent: Mix.env() == :prod,
        deps: deps(),
        description: description(),
      package: package(),
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
        {:rustler, "~> 0.31.0"},
        {:rustler_precompiled, "~> 0.7"}
      ]
    end

    defp description() do
    "Elixir bindings for the rustrict crate."
  end

  defp package() do
    [
      name: "bartender",
      licenses: ["MIT"],
      files: [
      "lib",
      "native/bartender/.cargo",
      "native/bartender/src",
      "native/bartender/Cargo*",
      "checksum-*.exs",
      "mix.exs"
    ],
      links: %{"GitHub" => "https://github.com/stalepretzels/bartender"}
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
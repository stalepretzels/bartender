defmodule Bartender.Mixfile do
    use Mix.Project
  
    def project do
      [
        app: :nif_bartender,
        version: "0.1.1",
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
        {:rustler, "~> 0.23.0"},
        {:ex_doc, ">= 0.0.0", only: :dev, runtime: false}
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
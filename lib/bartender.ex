  defmodule Bartender do
    use Rustler, otp_app: :nif_bartender, crate: :bartender
  
    def censor(_input), do: error()
    def is_inappropriate(_input), do: error()
    def is(_input, _filter_input), do: error()
    def isnt(_input, _filter_input), do: error()
  
    defp error, do: :erlang.nif_error(:nif_not_loaded)
  end
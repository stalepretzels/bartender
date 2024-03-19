defmodule BartenderTest do
    use ExUnit.Case
    doctest Bartender
  
    test "censor" do
      assert Bartender.censor("holy crap") == "holy c***"
    end

    test "is inappropriate" do
      assert Bartender.is_inappropriate("f u c k") == true
      assert Bartender.is_inappropriate("hello") == false
    end

    test "is" do
      assert Bartender.is("pron", "evasive") == true
      assert Bartender.is("porn", "evasive") == false

      assert Bartender.is("Hello there!", "safe") == true
      assert Bartender.is("nice work.", "safe") == true
      assert Bartender.is("yes", "safe") == true
      assert Bartender.is("NVM", "safe") == true
      assert Bartender.is("gtg", "safe") == true
      assert Bartender.is("not a common phrase", "safe") == false
    end

    test "isnt" do
      assert Bartender.isnt("pron", "evasive") == false
      assert Bartender.isnt("porn", "evasive") == true

      assert Bartender.isnt("Hello there!", "safe") == false
      assert Bartender.isnt("nice work.", "safe") == false
      assert Bartender.isnt("yes", "safe") == false
      assert Bartender.isnt("NVM", "safe") == false
      assert Bartender.isnt("gtg", "safe") == false
      assert Bartender.isnt("not a common phrase", "safe") == true
    end
  end
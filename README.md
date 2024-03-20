# Bartender
[![.github/workflows/release.yml](https://github.com/stalepretzels/bartender/actions/workflows/release.yml/badge.svg)](https://github.com/stalepretzels/bartender/actions/workflows/release.yml)

Elixir bindings for the `rustrict` crate.

**NOTE: `rustrict` was made by [@finnbear](https://github.com/finnbear), and not me! I am simply making bindings to it! Please check out [his original project](https://github.com/finnbear/rustrict).**

## How capable is this library?
Since it's based off of `rustrict`, it has every capability of the rust crate! Here's what the original can do:

<details>
<summary>Features</summary>

- Multiple types (profane, offensive, sexual, mean, spam)
- Multiple levels (mild, moderate, severe)
- Resistant to evasion
  - Alternative spellings (like "fck")
  - Repeated characters (like "craaaap")
  - Confusable characters (like 'ᑭ', '𝕡', and '🅿')
  - Spacing (like "c r_a-p")
  - Accents (like "pÓöp")
  - Bidirectional Unicode ([related reading](https://blog.rust-lang.org/2021/11/01/cve-2021-42574.html))
  - Self-censoring (like "f*ck")
  - Safe phrase list for known bad actors]
  - Censors invalid Unicode characters
  - Battle-tested in [Mk48.io](https://mk48.io)
- Resistant to false positives
  - One word (like "**ass**assin")
  - Two words (like "pu**sh it**")
- Flexible
  - Censor and/or analyze
  - Input `&str` or `Iterator<Item = char>`
  - Can track per-user state with `context` feature
  - Can add words with the `customize` feature
  - Accurately reports the width of Unicode via the `width` feature
  - Plenty of options
- Performant
  - O(n) analysis and censoring
  - No `regex` (uses custom trie)
  - 3 MB/s in `release` mode
  - 100 KB/s in `debug` mode

</details>

## Limitations
This is handicapped by everything the original was:
<details>
<summary>Limitations</summary>

- Mostly English/emoji
- Censoring removes most diacritics (accents)
- Does not detect right-to-left profanity while analyzing, so...
- Censoring forces Unicode to be left-to-right
- Doesn't understand context
- Not resistant to false positives affecting profanities added at runtime

</details>

## How do I use this?
Simple! Just alias (or import) Bartender at the beginning of the your `.ex` file to begin!
```elixir
alias Bartender
# or `import Bartender`
# Simply remove `Bartender.` from the beginning of the code if you plan on using import. 
```

Censor and check for profanity:
```elixir
# censor(input: String) -> String
# is_inappropriate(input: String) -> bool

censored = Bartender.censor("hello crap") # -> "hello c***"

inappropriate = Bartender.is_inappropriate("f u c k") # -> true
```

Type match strings:
```elixir
# is(input: String, filter_input: String) -> bool
# isnt(input: String, filter_input: String) -> bool

Bartender.is("pron", "evasive") # -> true
Bartender.isnt("porn", "evasive") # -> true

Bartender.is("Hello there!", "safe") # -> true
Bartender.is("nice work.", "safe") # -> true
Bartender.is("yes", "safe") # -> true
Bartender.is("NVM", "safe") # -> true
Bartender.is("gtg", "safe") # -> true
Bartender.isnt("not a common phrase", "safe") # -> true
```

## Roadmap
1. Add custom words and censors
2. Implement `context` feature
3. Array iterators

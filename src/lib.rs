use rustrict::CensorStr;

#[rustler::nif]
fn filter_profanity(input: String) -> String {
    input.censor()
}

rustler::init!("Elixir.Bartender.Filterer", [filter_profanity]);
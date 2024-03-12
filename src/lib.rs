use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};
use rustrict::CensorStr;

mod atoms {
    rustler::atoms! {
        ok,
        error,
    }
}

#[rustler::nif]
fn filter_profanity(input: String) -> NifResult<String> {
    let filtered_text = input.censor(input);
    Ok(filtered_text)
}

rustler::init!("Elixir.Bartender.Filterer", [filter_profanity], load = |env, _| {
    Ok(())
});
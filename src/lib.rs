use rustrict::{CensorStr, Type};
use std::collections::HashMap;
use once_cell::sync::Lazy;

static TYPE_MAP: Lazy<HashMap<String, Type>> = Lazy::new(|| HashMap::from(
[
    ("profane".to_string(), Type::PROFANE),
    ("offensive".to_string(), Type::OFFENSIVE),
    ("sexual".to_string(), Type::SEXUAL),
    ("mean".to_string(), Type::MEAN),
    ("evasive".to_string(), Type::EVASIVE),
    ("spam".to_string(), Type::SPAM),
    ("safe".to_string(), Type::SAFE),
    ("mild".to_string(), Type::MILD),
    ("moderate".to_string(), Type::MODERATE),
    ("severe".to_string(), Type::SEVERE),
    ("mild_or_higher".to_string(), Type::MILD_OR_HIGHER),
    ("moderate_or_higher".to_string(), Type::MODERATE_OR_HIGHER),
    ("inappropriate".to_string(), Type::INAPPROPRIATE),
    ("any".to_string(), Type::ANY),
    ("none".to_string(), Type::NONE),
]));

fn bitwise_equation(input: &str) -> Option<Type> {
    let keys: Vec<&str> = input.split(" | ").collect();

    if keys.is_empty() {
        return None;
    }

    let mut result = Type::NONE;

    for key in keys {
        if let Some(&value) = TYPE_MAP.get(key) {
            result |= value;
        } else {
            return None; // Key not found in the map
        }
    }

    Some(result)
}

#[rustler::nif]
fn censor(input: String) -> String {
    input.censor()
}

#[rustler::nif]
fn is_inappropriate(input: String) -> bool {
    input.is_inappropriate()
}

#[rustler::nif]
fn is(input: String, filter_input: String) -> bool {
    let filter;
    if let Some(result) = bitwise_equation(&filter_input) {
        filter = result;
    } else {
        panic!("Invalid input or keys not found in the map");
    }
    input.is(filter)
}

#[rustler::nif]
fn isnt(input: String, filter_input: String) -> bool {
    let filter;
    if let Some(result) = bitwise_equation(&filter_input) {
        filter = result;
    } else {
        panic!("Invalid input or keys not found in the map");
    }
    input.isnt(filter)
}

rustler::init!("Elixir.Bartender", [censor, is_inappropriate, is, isnt]);

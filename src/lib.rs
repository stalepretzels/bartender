use rustrict::{CensorStr, Type};
use std::collections::HashMap;

let mut type_map = HashMap::new();

    type_map.insert(
        "profane".to_string(),
        Type::PROFANE,
    );
    type_map.insert(
        "offensive".to_string(),
        Type::OFFENSIVE,
    );
    type_map.insert(
        "sexual".to_string(),
        Type::SEXUAL,
    );
    type_map.insert(
        "mean".to_string(),
        Type::MEAN,
    );
    type_map.insert(
        "evasive".to_string(),
        Type::EVASIVE,
    );
    type_map.insert(
        "spam".to_string(),
        Type::SPAM,
    );
    type_map.insert(
        "safe".to_string(),
        Type::SAFE,
    );
    type_map.insert(
        "mild".to_string(),
        Type::MILD,
    );
    type_map.insert(
        "moderate".to_string(),
        Type::MODERATE,
    );
    type_map.insert(
        "severe".to_string(),
        Type::SEVERE,
    );
    type_map.insert(
        "mild_or_higher".to_string(),
        Type::MILD_OR_HIGHER,
    );
    type_map.insert(
        "moderate_or_higher".to_string(),
        Type::MODERATE_OR_HIGHER,
    );
    type_map.insert(
        "inappropriate".to_string(),
        Type::INAPPROPRIATE,
    );
    type_map.insert(
        "any".to_string(),
        Type::ANY,
    );
    type_map.insert(
        "none".to_string(),
        Type::NONE,
    );

fn bitwise_equation(input: &str, map: &HashMap<String, Type>) -> Option<Type> {
    let keys: Vec<&str> = input.split(" | ").collect();

    if keys.is_empty() {
        return None;
    }

    let mut result = Type::NONE;

    for key in keys {
        if let Some(&value) = map.get(key) {
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
    let mut filter;
    if let Some(result) = bitwise_equation(filter_input) {
        filter = result;
    } else {
        panic!("Invalid input or keys not found in the map");
    }
    input.is(filter)
}

#[rustler::nif]
fn isnt(input: String, filter_input: String) -> bool {
    let mut filter;
    if let Some(result) = bitwise_equation(filter_input) {
        filter = result;
    } else {
        panic!("Invalid input or keys not found in the map");
    }
    input.isnt(filter)
}

rustler::init!("Elixir.Bartender", [censor, is_inappropriate, is, isnt]);

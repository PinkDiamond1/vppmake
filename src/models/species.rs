use super::types::Types;
use serde::Deserialize;
use smartstring::alias::String;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Species {
    slug: String,
    name: String,
    types: Types,
}

use serde::Deserialize;
use smol_str::SmolStr;

use super::Types;
#[derive(PartialEq, Debug, Deserialize)]
pub struct Species {
    slug: SmolStr,
    name: SmolStr,
    types: Types,
}

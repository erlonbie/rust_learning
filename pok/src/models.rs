use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub id: u64,
    pub name: String,
    pub types: Vec<TypeSlot>,
    pub stats: Vec<StatSlot>,
    pub height: f32,
    pub weight: f32,
}

#[derive(Deserialize, Debug)]
pub struct TypeSlot {
    #[allow(dead_code)]
    pub slot: u64,
    #[serde(rename = "type")]
    pub type_object: Type,
}

#[derive(Deserialize, Debug)]
pub struct StatSlot {
    pub base_stat: u64,
    #[allow(dead_code)]
    pub effort: u64,
    #[serde(rename = "stat")]
    pub stat_object: Stat,
}

#[derive(Deserialize, Debug)]
pub struct Stat {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Type {
    pub name: String,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Source {
  pub message: String,
  pub log_group: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Hit {
  pub _index :String,
  pub _type :String,
  pub _id :String,
  pub _score :f32,
  #[serde(rename = "_source")]
  pub source : Source,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Hits {
  #[serde(rename = "hits")]
  pub hit_list: Vec<Hit>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryResponse {
  #[serde(rename = "hits")]
  pub hits: Hits
}

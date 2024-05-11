use schemars::JsonSchema;
use serde::{Serialize , Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg{}

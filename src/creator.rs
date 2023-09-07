pub mod init_go_project;
pub mod populate_framework;
pub mod struct_creator;
pub use init_go_project::initialize_go_project;
pub use populate_framework::populate_framework;
use serde::{Deserialize, Serialize};
pub use struct_creator::generate_structure;

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub json_name: String,
    pub dynamo_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoStruct {
    pub project_name: String,
    pub name: String,
    pub fields: Vec<Field>,
}

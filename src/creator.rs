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
impl Field {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.chars().next().unwrap().is_lowercase() {
            return Err("Field name must start with an uppercase letter".to_string());
        }

        let allowed_types = vec!["string", "int", "float64", "bool", "byte"];
        if !allowed_types.contains(&self.field_type.as_str()) {
            return Err(format!(
                "Invalid field type: {}. Allowed types are {:?}",
                self.field_type, allowed_types
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoStruct {
    pub project_name: String,
    pub name: String,
    pub fields: Vec<Field>,
}

impl GoStruct {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.chars().next().unwrap().is_lowercase() {
            return Err("Struct name must start with an uppercase letter".to_string());
        }

        for field in &self.fields {
            if let Err(err) = field.validate() {
                return Err(err);
            }
        }

        Ok(())
    }
}

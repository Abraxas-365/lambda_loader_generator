pub mod init_go_project;
pub mod populate_framework;
pub mod struct_creator;
pub use init_go_project::initialize_go_project;
pub use populate_framework::populate_framework;
use serde::{Deserialize, Serialize};
pub use struct_creator::generate_structure;

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}
impl Struct {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: Option<String>,
    pub json_name: String,
    pub dynamo_name: String,
    pub is_slice: Option<bool>,
    pub nested_struct: Option<Box<Struct>>, // new field for nested struct
}
impl Field {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.chars().next().unwrap().is_lowercase() {
            return Err("Field name must start with an uppercase letter".to_string());
        }

        if self.field_type.is_none() && self.nested_struct.is_none() {
            return Err("Either 'type' or 'nested_struct' must be provided".to_string());
        }

        if let Some(field_type) = &self.field_type {
            let allowed_types = vec!["string", "int", "float64", "bool", "byte"];

            let type_to_check = field_type.as_str();

            if !allowed_types.contains(&type_to_check) {
                println!("Type to check: {}", type_to_check);

                return Err(format!(
                    "Invalid field type. Allowed types are {:?}",
                    allowed_types
                ));
            }
        }

        if let Some(nested_struct) = &self.nested_struct {
            if let Err(err) = nested_struct.validate() {
                return Err(format!("Nested struct validation failed: {}", err));
            }
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

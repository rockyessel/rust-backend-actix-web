use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Package  {
 pub language: String,
 pub name_of_package_library: String,
 pub version: String,
 pub description:String

}
// Language, name_of_package_library, version, description, code, user_who_added, user_who_made_changes, comments
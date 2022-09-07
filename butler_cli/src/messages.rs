use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Version {
    pub app_id: String,
    pub text: String,
}

impl Version {
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

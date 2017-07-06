use super::*;

#[derive(Serialize, Deserialize)]
pub struct IndexContext {
    pub name: String,
    pub title: String,
    pub parent: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterContext {
    pub title: String,
    pub errors: Option<String>,
    pub parent: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginContext {
    pub title: String,
    pub parent: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogoutContext {
    pub title: String,
    pub parent: String,
}

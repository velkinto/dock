use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PushData {
    pub digest: String,
    pub pushed_at: String,
    pub tag: String
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub date_created: String,
    pub name: String,
    pub namespace: String,
    pub region: String,
    pub repo_authentication_type: String,
    pub repo_full_name: String,
    pub repo_origin_type: String,
    pub repo_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Trigger {
    pub push_data: PushData,
    pub repository: Repository
}

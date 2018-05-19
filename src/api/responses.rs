#[derive(Serialize, Deserialize)]
pub struct Identity {
    pub user_id: i32,
    pub email: String,
    pub quota: i32,
    pub quota_ttl: i32,
}

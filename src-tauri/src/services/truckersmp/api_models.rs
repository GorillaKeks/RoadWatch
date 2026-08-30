use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub error: bool,
    pub response: T,
}

#[derive(Debug, Deserialize)]
pub struct VtcMembersResponse {
    #[serde(default)]
    pub members: Vec<VtcMember>,
}

#[derive(Debug, Deserialize)]
pub struct VtcMember {
    pub id: u64,

    #[serde(rename = "user_id")]
    pub user_id: u64,

    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub avatar: Option<String>,

    /// TruckersMP VTC rank.
    ///
    /// The API has used rank-related fields in different responses.
    /// We keep the possible values optional so older/incomplete
    /// responses continue to work.
    #[serde(default)]
    pub role: Option<String>,

    #[serde(default)]
    pub rank: Option<String>,

    #[serde(default)]
    pub rank_name: Option<String>,
}

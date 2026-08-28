use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vtc {
    pub id: u64,
    pub truckersmp_id: u64,
    pub name: String,
    pub tag: Option<String>,
    pub member_count: u32,
}

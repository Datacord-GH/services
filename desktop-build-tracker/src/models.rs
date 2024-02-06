#[derive(Debug)]
#[allow(dead_code)]
pub struct Build {
    pub id: i32,
    pub channel: String,
    pub build_number: String,
    pub build_hash: String,
    pub build_id: String,
}
